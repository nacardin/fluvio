//!
//! # Create Spu Groups Request
//!
//! Converts Spu Gruups API request into KV request and sends to KV store for processing.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;

use k8_metadata_client::MetadataClient;
use sc_api::FlvStatus;
use sc_api::spu::CreateSpuGroupRequest;
use sc_api::spu::SpuGroupSpec;


use super::PublicContext;

/// Handler for spu groups request
pub async fn handle_create_spu_group_request<C>(
    request: RequestMessage<CreateSpuGroupRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, Error>
where
    C: MetadataClient,
{
    let (header, req) = request.get_header_request();

    debug!("creating spg: {}", req.name);
    
   
    let status = process_custom_spu_request(ctx, req.name,req.spec).await;
    trace!("create spu-group response {:#?}", status);

    Ok(ResponseMessage::from_header(&header, status))
}

/// Process custom spu, converts spu spec to K8 and sends to KV store
async fn process_custom_spu_request<C>(
    ctx: &PublicContext<C>,
    name: String,
    spg_spec: SpuGroupSpec,
) -> FlvStatus
where
    C: MetadataClient,
{
    use k8_metadata::spg::SpuGroupSpec as K8SpuGroupspec;

    match ctx.create::<K8SpuGroupspec>(&name, spg_spec.into()).await {
        Ok(_) => FlvStatus::new_ok(name.clone()),
        Err(err) => {
            let error = Some(err.to_string());
            FlvStatus::new(name, FlvErrorCode::SpuError, error)
        }
    }
}
