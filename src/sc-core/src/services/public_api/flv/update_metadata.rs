use std::sync::Arc;
use std::time::Duration;

use log::debug;
use log::error;
use event_listener::Event;
use futures::io::AsyncRead;
use futures::io::AsyncWrite;

use kf_protocol::api::*;
use sc_api::metadata::*;
use kf_socket::*;
use flv_future_aio::zero_copy::ZeroCopyWrite;
use flv_future_aio::timer::sleep;
use flv_future_aio::sync::broadcast::*;

use crate::core::SharedContext;


/// metadata request are handle thru MetadataController which waits metadata event from ConnManager
/// and forward to Client


/// 
pub struct ClientMetadataController<S> {
    response_sink: InnerExclusiveKfSink<S>,
    context: SharedContext,
    metadata_request: UpdateMetadataRequest,
    header: RequestHeader,
    end_event: Arc<Event>
}

impl<S>  ClientMetadataController<S> 
    where S: AsyncWrite + AsyncRead + Unpin + Send + ZeroCopyWrite + 'static, 
{

    pub fn handle_metadata_update(
        request: RequestMessage<UpdateMetadataRequest>,
        response_sink: InnerExclusiveKfSink<S>,
        end_event: Arc<Event>,
        context: SharedContext,
    ) 
        
    {
        let (header,metadata_request) = request.get_header_request();
        let controller = Self {
            response_sink,
            context,
            header,
            metadata_request,
            end_event
        };

        controller.run();        
            
    }

    /// send out all metadata to client
    async fn update_all(&mut self) -> Result<(),KfSocketError> {

        
        let spu_specs = self.context.spus().all_specs();
        let partitions = self.context.partitions().leaders();
         
        let response = UpdateAllMetadataResponse::new(spu_specs, partitions);
        
        self.response_sink.send_response(&ResponseMessage::new(self.header.correlation_id(), response), self.header.api_version()).await

    }

    pub fn run(self) {

        use flv_future_aio::task::spawn;

        spawn(self.dispatch_loop());
    }
   
    async fn dispatch_loop(mut self) {

        use tokio::select;

        let mut counter: i32 = 0;
        let mut receiver = self.context.new_client_subscriber();
        let sink_id = self.response_sink.id();
        let correlation_id = self.header.correlation_id();

        // first send everything
        if let Err(err) = self.update_all().await {
            error!("error updating all schema: {}, error: {}",sink_id,err);   
        }

        loop {

            counter += 1;
            debug!("waiting on conn: {}: correlation: {}, counter: {}",
                sink_id,
                correlation_id,
                counter
            );


            select! {

                _ = (sleep(Duration::from_secs(60))) => {

                    debug!("metadata reconcillation: {}, correlation: {}",sink_id,correlation_id);
                    if let Err(err) = self.update_all().await {
                        error!("error updating all schema: {}, error: {}",sink_id,err);   
                    }

                },
                client_event = receiver.recv() => {

                    match client_event {
                        Ok(value) => {

                            use crate::controllers::conn_manager::*;

                            match value {
                                ClientNotification::SPU(spu) => {
                                    let response = UpdateSpuResponse::new(spu);
                                },
                                ClientNotification::Partition(partition) => {
                                    let response = UpdateReplicaResponse::new(partition);
                                }
                                
                            };

                        },
                        Err(err) => {
                            match err {
                                RecvError::Closed => {
                                    error!("receiver to conn manager closed!");
                                },
                                RecvError::Lagged(lag) => {
                                    error!("conn: {}, lagging: {}",sink_id,lag);
                                }
                            
                            }
                        }
                    }
                    
                },

                _ = self.end_event.listen() => {
                    debug!("socket: {}, terminated, ending loop",sink_id);
                    break;
                }


             }

        }
    }


    
}

