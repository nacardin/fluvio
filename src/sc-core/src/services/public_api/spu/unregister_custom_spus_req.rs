//!
//! # Delete Custom Spus Request
//!
//! Lookup custom-spu in local metadata, grab its K8 context
//! and send K8 a delete message.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;
use sc_api::FlvStatus;
use sc_api::spu::*;
use flv_metadata::spu::SpuSpec;
use k8_metadata_client::MetadataClient;

use crate::stores::spu::*;
use super::PublicContext;

/// Handler for delete custom spu request
pub async fn handle_unregister_custom_spu_request<C>(
    key: CustomSpuKey,
    ctx: &PublicContext<C>,
) -> Result<FlvStatus,Error>
where
    C: MetadataClient,
{
   
    let status = match key {
        CustomSpuKey::Name(spu_name) => {
            debug!("api request: delete custom-spu with name '{}'", spu_name);

            // spu-name must exist
            if let Some(spu) = &ctx.context().spus().spu(&spu_name) {
                unregister_custom_spu(ctx, spu).await?
            } else {
                // spu does not exist
                FlvStatus::new(
                    spu_name.clone(),
                    FlvErrorCode::SpuNotFound,
                    Some("not found".to_owned()),
                )
            }
        }
        CustomSpuKey::Id(spu_id) => {
            debug!("api request: delete custom-spu with id '{}'", spu_id);

            // spu-id must exist
            if let Some(spu) = &ctx.context().spus().get_by_id(&spu_id) {
                unregister_custom_spu(ctx, spu).await?
            } else {
                // spu does not exist
                FlvStatus::new(
                    format!("spu-{}", spu_id),
                    FlvErrorCode::SpuNotFound,
                    Some("not found".to_owned()),
                )
            }
        }
    };

    trace!("flv delete custom-spus resp {:#?}", status);

    Ok(status)
}

/// Generate for delete custom spu operation and return result.
pub async fn unregister_custom_spu<C>(
    ctx: &PublicContext<C>,
    spu: &SpuKV,
) -> Result<FlvStatus, Error>
where
    C: MetadataClient,
{
    let spu_name = spu.name();

    // must be Custom Spu
    if !spu.is_custom() {
        return Ok(FlvStatus::new(
            spu_name.clone(),
            FlvErrorCode::SpuError,
            Some("expected 'Custom' spu, found 'Managed' spu".to_owned()),
        ));
    }

    // have have KV context
    let item_ctx = match &spu.kv_ctx().item_ctx {
        Some(ctx) => ctx,
        None => {
            return Ok(FlvStatus::new(
                spu_name.clone(),
                FlvErrorCode::SpuError,
                Some("missing Kv context".to_owned()),
            ))
        }
    };

    // delete custom spec and return result
    let item = item_ctx.as_input();
    match ctx.k8_client().delete_item::<SpuSpec, _>(&item).await {
        Ok(_) => Ok(FlvStatus::new_ok(spu_name.clone())),
        Err(err) => Ok(FlvStatus::new(
            spu_name.clone(),
            FlvErrorCode::SpuError,
            Some(err.to_string()),
        )),
    }
}
