//!
//! # Delete Custom Spus Request
//!
//! Lookup custom-spu in local metadata, grab its K8 context
//! and send K8 a delete message.
//!
use tracing::{debug, trace};
use std::io::{Error, ErrorKind};

use dataplane::ErrorCode;
use fluvio_sc_schema::Status;
use fluvio_sc_schema::spu::*;
use fluvio_service::auth::Authorization;

use crate::stores::spu::*;
use crate::core::*;
use crate::services::auth::basic::{Action, Object};

/// Handler for delete custom spu request
pub async fn handle_un_register_custom_spu_request(
    key: CustomSpuKey,
    auth_ctx: &AuthenticatedContext,
) -> Result<Status, Error> {
    let auth_request = (Action::Delete, Object::CustomSpu, None);
    if let Ok(authorized) = auth_ctx.auth.enforce(auth_request).await {
        if !authorized {
            trace!("authorization failed");
            let name: String = String::from(&key);
            return Ok(Status::new(
                name,
                ErrorCode::PermissionDenied,
                Some(String::from("permission denied")),
            ));
        }
    } else {
        return Err(Error::new(ErrorKind::Interrupted, "authorization io error"));
    }

    let spu_store = auth_ctx.global_ctx.spus().store();
    let status = match key {
        CustomSpuKey::Name(spu_name) => {
            debug!("api request: delete custom-spu with name '{}'", spu_name);

            // spu-name must exist
            if let Some(spu) = spu_store.value(&spu_name).await {
                un_register_custom_spu(&auth_ctx, spu.inner_owned()).await
            } else {
                // spu does not exist
                Status::new(
                    spu_name.clone(),
                    ErrorCode::SpuNotFound,
                    Some("not found".to_owned()),
                )
            }
        }
        CustomSpuKey::Id(spu_id) => {
            debug!("api request: delete custom-spu with id '{}'", spu_id);

            // spu-id must exist
            if let Some(spu) = spu_store.get_by_id(spu_id).await {
                un_register_custom_spu(&auth_ctx, spu).await
            } else {
                // spu does not exist
                Status::new(
                    format!("spu-{}", spu_id),
                    ErrorCode::SpuNotFound,
                    Some("not found".to_owned()),
                )
            }
        }
    };

    trace!("flv delete custom-spus resp {:#?}", status);

    Ok(status)
}

/// Generate for delete custom spu operation and return result.
async fn un_register_custom_spu(auth_ctx: &AuthenticatedContext, spu: SpuAdminMd) -> Status {
    let spu_name = spu.key_owned();

    // must be Custom Spu
    if !spu.spec.is_custom() {
        return Status::new(
            spu_name,
            ErrorCode::SpuError,
            Some("expected 'Custom' spu, found 'Managed' spu".to_owned()),
        );
    }

    // delete custom spec and return result
    if let Err(err) = auth_ctx.global_ctx.spus().delete(spu_name.clone()).await {
        Status::new(
            spu_name,
            ErrorCode::SpuError,
            Some(format!("error deleting: {}", err)),
        )
    } else {
        Status::new_ok(spu_name.clone())
    }
}
