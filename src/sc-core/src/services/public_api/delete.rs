//!
//! # Delete Topic Request
//!
//! Delete topic request handler. Lookup topic in local metadata, grab its K8 context
//! and send K8 a delete message.
//!
use log::trace;
use std::io::Error;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use sc_api::FlvStatus;
use sc_api::objects::*;
use k8_metadata_client::MetadataClient;

use super::PublicContext;

/// Handler for delete topic request
pub async fn handle_delete_request<C>(
    request: RequestMessage<DeleteRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, Error>
where
    C: MetadataClient,
{
    let (header, req) = request.get_header_request();
   
    let status = match req {
        DeleteRequest::Topic(name) => {
            super::topic::handle_delete_topic(name, ctx).await?
        },
        DeleteRequest::CustomSpu(key) => {
            super::spu::handle_unregister_custom_spu_request(key, ctx).await?
        },
        DeleteRequest::SpuGroup(name) => {
            super::spg::handle_delete_spu_group(name, ctx).await?
        }
    };

       
    trace!("flv delete topics resp {:#?}", status);

    Ok(ResponseMessage::from_header(&header, status))
}
