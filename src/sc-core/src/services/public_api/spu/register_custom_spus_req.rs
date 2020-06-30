//!
//! # Create Custom Spus Request
//!
//! Converts Custom Spu API request into KV request and sends to KV store for processing.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::FlvErrorCode;

use flv_metadata::k8::metadata::ObjectMeta;
use k8_metadata_client::MetadataClient;

use sc_api::FlvStatus;
use sc_api::spu::CustomSpuSpec;
use sc_api::spu::SpuSpec;
use crate::core::Context;
use crate::stores::spu::*;
use crate::stores::*;

use super::PublicContext;

/// Handler for create spus request
pub async fn handle_register_custom_spu_request<C>(
    name: String,
    spec: CustomSpuSpec,
    dry_run: bool,
    ctx: &PublicContext<C>,
) -> Result<FlvStatus, Error>
where
    C: MetadataClient,
{
    debug!("api request: create custom-spu '{}({})'", name, spec.id);

    let mut status = FlvStatus::default();

    // validate custom-spu request
    if let Err(err) = validate_custom_spu_request(spec.id, &name, ctx.context()).await {
        status = err;
    }

    if !dry_run {
        status = process_custom_spu_request(ctx, &name, spec).await;
    }

    trace!("create custom-spus response {:#?}", status);

    Ok(status)
}

/// Validate custom_spu requests (one at a time)
async fn validate_custom_spu_request(
    spu_id: i32,
    spu_name: &str,
    metadata: &Context,
) -> Result<(), FlvStatus> {
    debug!("validating custom-spu: {}({})", spu_name, spu_id);

    // look-up SPU by name or id to check if already exists
    if metadata.spus().spu(spu_name).await.is_some()
        || metadata.spus().get_by_id(spu_id).await.is_some()
    {
        Err(FlvStatus::new(
            spu_name.to_owned(),
            FlvErrorCode::SpuAlreadyExists,
            Some(format!("spu '{}({})' already defined", spu_name, spu_id)),
        ))
    } else {
        Ok(())
    }
}

/// Process custom spu, converts spu spec to K8 and sends to KV store
async fn process_custom_spu_request<C>(
    ctx: &PublicContext<C>,
    name: &str,
    custom_spu: CustomSpuSpec,
) -> FlvStatus
where
    C: MetadataClient,
{
    if let Err(err) = register_custom_spu(ctx, name, custom_spu).await {
        let error = Some(err.to_string());
        FlvStatus::new(name.to_owned(), FlvErrorCode::SpuError, error)
    } else {
        FlvStatus::new_ok(name.to_owned())
    }
}

/// register custom spu by convert into spu spec since custom spec is just subset
async fn register_custom_spu<C>(
    ctx: &PublicContext<C>,
    name: &str,
    spu_spec: CustomSpuSpec,
) -> Result<(), C::MetadataClientError>
where
    C: MetadataClient,
{
    let meta = ObjectMeta::new(name.clone(), &ctx.namespace);
    let spu_spec: SpuSpec = spu_spec.into();
    let kv_ctx = MetadataContext::default().with_ctx(meta);
    let custom_spu_kv = SpuKV::new_with_context(name.clone(), spu_spec, kv_ctx);

    ctx.k8_ws().add(custom_spu_kv).await
}
