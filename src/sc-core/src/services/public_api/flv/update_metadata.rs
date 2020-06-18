use log::debug;
use log::error;
use futures::io::*;
use async_trait::async_trait;
use tokio::select;

use kf_protocol::api::RequestMessage;
use sc_api::metadata::*;
use kf_socket::InnerExclusiveKfSink;
use flv_future_aio::zero_copy::ZeroCopyWrite;
use flv_future_aio::actor::AsyncDispatcher;
use flv_future_aio::sync::broadcast::*;

use crate::core::Context;
use crate::controllers::ClientNotification;

/// metadata request are handle thru MetadataController which waits metadata event from ConnManager
/// and forward to Client


/// 
pub struct ClientMetadataController<S> {
    response_sink: InnerExclusiveKfSink<S>,
    client_receiver: Receiver<ClientNotification>,
    correlation_id: i32

}

impl<S>  ClientMetadataController<S> 
    where S: AsyncWrite + AsyncRead + Unpin + Send + ZeroCopyWrite + 'static, 
{

    pub fn handle_metadata_update(
        request: RequestMessage<UpdateMetadataRequest>,
        response_sink: InnerExclusiveKfSink<S>,
        context: &Context
    ) 
        
    {
        let controller = Self {
            response_sink,
            client_receiver: context.new_client_subscriber(),
            correlation_id: request.get_header_request().0.correlation_id()
        };

        controller.run();
            
    }
}

#[async_trait]
impl<S> AsyncDispatcher for ClientMetadataController<S>
    where S: AsyncWrite + AsyncRead + Unpin + Send + ZeroCopyWrite + 'static, 
{

   
    async fn dispatch_loop(mut self) {

        let mut counter: i32 = 0;

        loop {

            debug!("waiting on conn: {}: correlation: {}, counter: {}",
                self.response_sink.id(),
                self.correlation_id,
                counter
            );

            match self.client_receiver.recv().await {
                Ok(value) => {},
                Err(err) => {
                    match err {
                        RecvError::Closed => {
                            error!("recever to conn manager closed!");
                        },
                        RecvError::Lagged(lag) => {
                            error!("conn: {}, lagging: {}",self.response_sink.id(),lag);
                        }
                    
                    }
                }
             }

        }

        
    }
}

