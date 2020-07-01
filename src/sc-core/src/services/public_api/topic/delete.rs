//!
//! # Delete Topic Request
//!
//! Delete topic request handler. Lookup topic in local metadata, grab its K8 context
//! and send K8 a delete message.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::FlvErrorCode;
use sc_api::FlvStatus;
use flv_metadata::topic::TopicSpec;
use k8_metadata_client::MetadataClient;

use super::PublicContext;

/// Handler for delete topic request
pub async fn handle_delete_topic<C>(
    topic_name: String,
    ctx: &PublicContext<C>,
) -> Result<FlvStatus, Error>
where
    C: MetadataClient,
{
   
    debug!("api request: delete topic '{}'", topic_name);

    let status = if let Some(topic) = ctx.context().topics().topic(&topic_name).await {
        
        let item = topic.ctx().item().as_input();
        if let Err(err) = ctx.k8_client().delete_item::<TopicSpec, _>(&item).await {
            FlvStatus::new(
                topic_name.clone(),
                FlvErrorCode::TopicError,
                Some(err.to_string()),
            )
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

    Ok(status)
}
