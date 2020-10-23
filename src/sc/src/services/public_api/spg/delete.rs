use tracing::debug;
use tracing::trace;

use std::io::{Error, ErrorKind};

use fluvio_service::auth::Authorization;
use fluvio_sc_schema::Status;

use crate::core::*;
use crate::services::auth::basic::{Action, Object};

/// Handler for delete spu group request
pub async fn handle_delete_spu_group(
    name: String,
    auth_ctx: &AuthenticatedContext,
) -> Result<Status, Error> {
    use dataplane::ErrorCode;

    debug!("delete spg group: {}", name);

    let auth_request = (Action::Delete, Object::SpuGroup, None);
    if let Ok(authorized) = auth_ctx.auth.enforce(auth_request).await {
        if !authorized {
            trace!("authorization failed");
            return Ok(Status::new(
                name.clone(),
                ErrorCode::PermissionDenied,
                Some(String::from("permission denied")),
            ));
        }
    } else {
        return Err(Error::new(ErrorKind::Interrupted, "authorization io error"));
    }

    let status = if auth_ctx
        .global_ctx
        .spgs()
        .store()
        .value(&name)
        .await
        .is_some()
    {
        if let Err(err) = auth_ctx.global_ctx.spgs().delete(name.clone()).await {
            Status::new(name.clone(), ErrorCode::SpuError, Some(err.to_string()))
        } else {
            Status::new_ok(name)
        }
    } else {
        Status::new(name, ErrorCode::SpuNotFound, Some("not found".to_owned()))
    };

    trace!("flv delete spu group resp {:#?}", status);

    Ok(status)
}
