use futures::io::*;

use kf_protocol::api::RequestMessage;
use sc_api::metadata::*;
use kf_socket::InnerExclusiveKfSink;
use flv_future_aio::zero_copy::ZeroCopyWrite;
use flv_future_aio::task::spawn;

use crate::core::Context;


/// metadata request are handle thru MetadataController which waits metadata event from ConnManager
/// and forward to Client


/// 
pub struct ClientMetadataController<S> {
    response_sink: InnerExclusiveKfSink<S>
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
            response_sink
        };
            
    }

    pub fn run(self) {

    }


    async fn dispatch_loop(mut self) {
        
    }
}

