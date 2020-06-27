//!
//! # Create Spu Groups Request
//!
//! Converts Spu Gruups API request into KV request and sends to KV store for processing.
//!

use std::io::Error;

use log::{debug, trace};

use kf_protocol::api::FlvErrorCode;

use k8_metadata_client::MetadataClient;
use sc_api::FlvStatus;
use sc_api::spg::*;


use super::PublicContext;

/// Handler for spu groups request
pub async fn handle_create_spu_group_request<C>(
    name: String,
    spec: SpuGroupSpec,
    _dry_run: bool,
    ctx: &PublicContext<C>,
) -> Result<FlvStatus,Error>
where
    C: MetadataClient,
{
    
    debug!("creating spu group: {}", name);
    

    let status = process_custom_spu_request(ctx, name,spec).await;
    trace!("create spu-group response {:#?}", status);

    Ok(status)
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
 
    match ctx.create::<K8SpuGroupSpec>(&name, spg_spec.into()).await {
        Ok(_) => FlvStatus::new_ok(name.clone()),
        Err(err) => {
            let error = Some(err.to_string());
            FlvStatus::new(name, FlvErrorCode::SpuError, error)
        }
    }
}
