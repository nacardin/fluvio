//!
//! # Create Custom Spus Request
//!
//! Converts Custom Spu API request into KV request and sends to KV store for processing.
//!
use log::{debug, trace};
use std::io::Error;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;

use flv_metadata::k8::metadata::ObjectMeta;
use k8_metadata_client::MetadataClient;

use sc_api::FlvStatus;
use sc_api::spu::*;
use crate::core::Context;
use crate::stores::spu::*;
use crate::stores::*;


use super::PublicContext;

/// Handler for create spus request
pub async fn handle_register_custom_spu_request<C>(
    request: RequestMessage<RegisterCustomSpuRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, Error>
where
    C: MetadataClient,
{
    let (header, req) = request.get_header_request();

    debug!(
        "api request: create custom-spu '{}({})'",
        req.name, req.spec.id
    );

    let mut status = FlvStatus::default();

    // validate custom-spu request
    if let Err(err) = validate_custom_spu_request(&req, ctx.context()) {
        status = err;
    }

    if req.dry_run {
        status = process_custom_spu_request(ctx, req).await;

    }

    trace!("create custom-spus response {:#?}", status);

    Ok(ResponseMessage::from_header(&header, status))
}

/// Validate custom_spu requests (one at a time)
fn validate_custom_spu_request(
    custom_spu_req: &RegisterCustomSpuRequest,
    metadata: &Context,
) -> Result<(), FlvStatus> {
    let spu_id = &custom_spu_req.spec.id;
    let spu_name = &custom_spu_req.name;

    debug!("validating custom-spu: {}({})", spu_name, spu_id);

    // look-up SPU by name or id to check if already exists
    if metadata.spus().spu(spu_name).is_some() || metadata.spus().get_by_id(spu_id).is_some() {
        Err(FlvStatus::new(
            spu_name.clone(),
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
    custom_spu_req: RegisterCustomSpuRequest,
) -> FlvStatus
where
    C: MetadataClient,
{
    let name = custom_spu_req.name.clone();

    if let Err(err) = register_custom_spu(ctx, custom_spu_req).await {
        let error = Some(err.to_string());
        FlvStatus::new(name, FlvErrorCode::SpuError, error)
    } else {
        FlvStatus::new_ok(name)
    }
}

async fn register_custom_spu<C>(
    ctx: &PublicContext<C>,
    spu_req: RegisterCustomSpuRequest,
) -> Result<(), C::MetadataClientError>
where
    C: MetadataClient,
{
    let meta = ObjectMeta::new(spu_req.name.clone(), ctx.namespace.clone());
    let spu_spec = spu_req.spec.into();
    let kv_ctx = KvContext::default().with_ctx(meta);
    let custom_spu_kv = SpuKV::new_with_context(spu_req.name.clone(), spu_spec, kv_ctx);

    ctx.k8_ws().add(custom_spu_kv).await
}
