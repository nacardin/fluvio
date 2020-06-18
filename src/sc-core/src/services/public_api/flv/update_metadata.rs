use std::sync::Arc;
use std::time::Duration;

use log::debug;
use log::error;
use futures::io::*;
use async_trait::async_trait;
use tokio::select;
use event_listener::Event;

use kf_protocol::api::RequestMessage;
use sc_api::metadata::*;
use kf_socket::InnerExclusiveKfSink;
use flv_future_aio::zero_copy::ZeroCopyWrite;
use flv_future_aio::actor::AsyncDispatcher;
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
    correlation_id: i32,
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
            correlation_id: header.correlation_id(),
            metadata_request,
            end_event
        };

        controller.run();        
            
    }

    /// send out all metadata to client
    async fn update_all(&self) {

        /*
        let spu_specs = self.context.spus().all_specs();
        let partitions = self.context
            .partitions()
            .all_specs()
            .into_iter()
            .map(|partition_spec| ReplicaLeader { id: partition_spec.key, leader: partition_spec.leader })
            .collect();
        
        let response = UpdateAllMetadataResponse::new(spu_specs, partitions);
        */

    }
}

#[async_trait]
impl<S> AsyncDispatcher for ClientMetadataController<S>
    where S: AsyncWrite + AsyncRead + Unpin + Send + ZeroCopyWrite + 'static, 
{

   
    async fn dispatch_loop(mut self) {

        let mut counter: i32 = 0;
        let mut receiver = self.context.new_client_subscriber();
        let sink_id = self.response_sink.id();
        let correlation_id = self.correlation_id;

        // first send everything


        loop {

            counter += 1;
            debug!("waiting on conn: {}: correlation: {}, counter: {}",
                sink_id,
                self.correlation_id,
                counter
            );


            select! {

                _ = (sleep(Duration::from_secs(60))) => {

                },
                client_event = receiver.recv() => {

                    
                    match client_event {
                        Ok(value) => {},
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
                    
                }
             }

        }
    }


    
}

