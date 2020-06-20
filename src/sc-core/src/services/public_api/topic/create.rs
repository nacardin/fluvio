//!
//! # Create Topic Request
//!
//! Create topic request handler. There are 2 types of topics:
//!  * Topics with Computed Replicas (aka. Computed Topics)
//!  * Topics with Assigned Replicas (aka. Assigned Topics)
//!
//! Computed Topics use Fluvio algorithm for replica assignment.
//! Assigned Topics allow the users to apply their custom-defined replica assignment.
//!

use std::io::Error;

use log::{debug, trace};

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;

use k8_metadata::metadata::ObjectMeta;
use k8_metadata_client::MetadataClient;

use sc_api::FlvStatus;
use sc_api::topics::*;

use flv_metadata::topic::TopicSpec;

use crate::core::Context;
use crate::stores::topic::*;
use crate::stores::*;

use super::PublicContext;

/// Handler for create topic request
pub async fn handle_create_topics_request<C>(
    request: RequestMessage<FlvCreateTopicRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, Error>
where
    C: MetadataClient,
{
    let (header, req) = request.get_header_request();

    let validate_only = req.validate_only;
  

    let name = req.name;
    let topic_spec = req.spec;
    debug!("api request: create topic '{}'", name);

    // validate topic request
    let mut status = validate_topic_request(&name, &topic_spec, ctx.context());
    if !validate_only {
        status = process_topic_request(ctx, name, topic_spec).await;
    }

    trace!("create topics request response {:#?}", status);

    Ok(ResponseMessage::from_header(&header, status))
}

/// Validate topic, takes advantage of the validation routines inside topic action workflow
fn validate_topic_request(
    name: &str,
    topic_spec: &TopicSpec,
    metadata: &Context,
) -> Result<(), FlvStatus> {
    debug!("validating topic: {}", name);

    // check if topic already exists
    if metadata.topics().contains_key(name) {
        return Err(FlvStatus::new(
            name.to_string(),
            FlvErrorCode::TopicAlreadyExists,
            Some(format!("topic '{}' already defined", name)),
        ));
    }

    // create temporary topic status to return validation result
    let topic_kv = TopicKV::with_spec(name.to_owned(), topic_spec.clone());
    match topic_spec {
        TopicSpec::Computed(param) => {
            let next_state = topic_kv.validate_computed_topic_parameters(param);
            trace!("validating, computed topic: {:#?}", next_state);
            if next_state.resolution.is_invalid() {
                Err(FlvStatus::new(
                    name.to_string(),
                    FlvErrorCode::TopicError,
                    Some(next_state.reason),
                ))
            } else {
                let next_state = topic_kv.generate_replica_map(metadata.spus(), param);
                trace!("validating, generate replica map topic: {:#?}", next_state);
                if next_state.resolution.no_resource() {
                    Err(FlvStatus::new(
                        name.to_string(),
                        FlvErrorCode::TopicError,
                        Some(next_state.reason),
                    ))
                } else {
                    Ok(())
                }
            }
        }
        TopicSpec::Assigned(ref partition_map) => {
            let next_state = topic_kv.validate_assigned_topic_parameters(partition_map);
            trace!("validating, computed topic: {:#?}", next_state);
            if next_state.resolution.is_invalid() {
                Err(FlvStatus::new(
                    name.to_string(),
                    FlvErrorCode::TopicError,
                    Some(next_state.reason),
                ))
            } else {
                let next_state =
                    topic_kv.update_replica_map_for_assigned_topic(partition_map, metadata.spus());
                trace!("validating, assign replica map topic: {:#?}", next_state);
                if next_state.resolution.is_invalid() {
                    Err(FlvStatus::new(
                        name.to_string(),
                        FlvErrorCode::TopicError,
                        Some(next_state.reason),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// Process topic, converts topic spec to K8 and sends to KV store
async fn process_topic_request<C>(
    ctx: &PublicContext<C>,
    name: String,
    topic_spec: TopicSpec,
) -> FlvStatus
where
    C: MetadataClient,
{
    if let Err(err) = create_topic(ctx, name.clone(), topic_spec).await {
        let error = Some(err.to_string());
        FlvStatus::new(name, FlvErrorCode::TopicError, error)
    } else {
        FlvStatus::new_ok(name)
    }
}

async fn create_topic<C>(
    ctx: &PublicContext<C>,
    name: String,
    topic: TopicSpec,
) -> Result<(), C::MetadataClientError>
where
    C: MetadataClient,
{
    let meta = ObjectMeta::new(name.clone(), ctx.namespace.clone());
    let kv_ctx = KvContext::default().with_ctx(meta);
    let topic_kv = TopicKV::new_with_context(name, topic, kv_ctx);

    ctx.k8_ws().add(topic_kv).await
}
