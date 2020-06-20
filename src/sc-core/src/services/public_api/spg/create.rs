//!
//! # Create Spu Groups Request
//!
//! Converts Spu Gruups API request into KV request and sends to KV store for processing.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;
use k8_metadata::spg::SpuGroupSpec as K8SpuGroupspec;
use k8_metadata::spg::SpuTemplate as K8SpuTemplate;
use k8_metadata::spg::StorageConfig as K8StorageConfig;
use k8_metadata::metadata::Env as K8Env;
use k8_metadata_client::MetadataClient;
use k8_metadata::metadata::Spec as K8Spec;
use k8_metadata::metadata::TemplateSpec as K8TemplateSpec;
use sc_api::FlvResponseMessage;
use sc_api::spu::{FlvCreateSpuGroupsRequest, FlvCreateSpuGroupsResponse};
use sc_api::spu::SpuGroupSpec;
use sc_api::spu::EnvVar;
use sc_api::spu::StorageConfig;

use super::PublicContext;

/// Handler for spu groups request
pub async fn handle_create_spu_groups_request<C>(
    request: RequestMessage<FlvCreateSpuGroupsRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvCreateSpuGroupsResponse>, Error>
where
    C: MetadataClient,
{
    let (header, spu_group_req) = request.get_header_request();

    let mut results: Vec<FlvResponseMessage> = vec![];

    // process create spu groups requests in sequence
    for spu_group in spu_group_req.spu_groups {
        debug!("api request: create spu-group '{}'", spu_group.name);

        let result = process_custom_spu_request(ctx, spu_group).await;
        results.push(result);
    }

    // send response
    let mut response = FlvCreateSpuGroupsResponse::default();
    response.results = results;
    trace!("create spu-group response {:#?}", response);

    Ok(RequestMessage::<FlvCreateSpuGroupsRequest>::response_with_header(&header, response))
}

/// Process custom spu, converts spu spec to K8 and sends to KV store
async fn process_custom_spu_request<C>(
    ctx: &PublicContext<C>,
    group_req: SpuGroupSpec,
) -> FlvResponseMessage
where
    C: MetadataClient,
{
    let (name, spg_spec): (String, SpuGroupSpec) = group_req.to_spec();

    match ctx.create(name.clone(), spg_spec).await {
        Ok(_) => FlvResponseMessage::new_ok(name.clone()),
        Err(err) => {
            let error = Some(err.to_string());
            FlvResponseMessage::new(name, FlvErrorCode::SpuError, error)
        }
    }
}
