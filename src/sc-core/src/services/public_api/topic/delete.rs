//!
//! # Delete Topic Request
//!
//! Delete topic request handler. Lookup topic in local metadata, grab its K8 context
//! and send K8 a delete message.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;
use sc_api::FlvStatus;
use sc_api::topics::*;
use k8_metadata::topic::TopicSpec as K8TopicSpec;
use k8_metadata_client::MetadataClient;

use super::PublicContext;

/// Handler for delete topic request
pub async fn handle_delete_topic_request<C>(
    request: RequestMessage<FlvDeleteTopicRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, Error>
where
    C: MetadataClient,
{
    let (header, req) = request.get_header_request();
   
    
    let topic_name = &req.name;
    debug!("api request: delete topic '{}'", topic_name);

    let status = if let Some(topic) = ctx.context().topics().topic(topic_name) {
        if let Some(item_ctx) = &topic.kv_ctx().item_ctx {
            let item = item_ctx.as_input();
            if let Err(err) = ctx.k8_client().delete_item::<K8TopicSpec, _>(&item).await {
                FlvStatus::new(
                    topic_name.clone(),
                    FlvErrorCode::TopicError,
                    Some(err.to_string()),
                )
            } else {
                FlvStatus::new_ok(topic_name.clone())
            }
        } else {
            FlvStatus::new_ok(topic_name.clone())
        }
    } else {
        // topic does not exist
        FlvStatus::new(
            topic_name.clone(),
            FlvErrorCode::TopicNotFound,
            Some("not found".to_owned()),
        )
    };
       
    trace!("flv delete topics resp {:#?}", status);

    Ok(ResponseMessage::from_header(&header, status))
}
