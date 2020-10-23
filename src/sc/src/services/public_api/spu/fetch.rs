use std::io::{Error, ErrorKind};

use tracing::{trace, debug};

use fluvio_service::auth::Authorization;
use fluvio_sc_schema::objects::*;
use fluvio_sc_schema::spu::SpuSpec;
use fluvio_sc_schema::spu::CustomSpuSpec;
use fluvio_controlplane_metadata::store::*;

use crate::core::AuthenticatedContext;
use crate::services::auth::basic::{Action, Object};

pub async fn handle_fetch_custom_spu_request(
    filters: Vec<String>,
    auth_ctx: &AuthenticatedContext,
) -> Result<ListResponse, Error> {
    debug!("fetching custom spu list");

    let auth_request = (Action::Read, Object::CustomSpu, None);
    if let Ok(authorized) = auth_ctx.auth.enforce(auth_request).await {
        if !authorized {
            trace!("authorization failed");
            // If permission denied, return empty list;
            return Ok(ListResponse::CustomSpu(vec![]));
        }
    } else {
        return Err(Error::new(ErrorKind::Interrupted, "authorization io error"));
    }

    let spus: Vec<Metadata<SpuSpec>> = auth_ctx
        .global_ctx
        .spus()
        .store()
        .read()
        .await
        .values()
        .filter_map(|value| {
            if value.spec().is_custom() && filters.filter(value.key()) {
                Some(value.inner().clone().into())
            } else {
                None
            }
        })
        .collect();

    let custom_spus: Vec<Metadata<CustomSpuSpec>> = spus
        .into_iter()
        .map(|spu| Metadata {
            name: spu.name,
            spec: spu.spec.into(),
            status: spu.status,
        })
        .collect();

    debug!("flv fetch custom resp: {} items", custom_spus.len());
    trace!("flv fetch custom spus resp {:#?}", custom_spus);

    Ok(ListResponse::CustomSpu(custom_spus))
}

pub async fn handle_fetch_spus_request(
    filters: Vec<String>,
    auth_ctx: &AuthenticatedContext,
) -> Result<ListResponse, Error> {
    debug!("fetching spu list");

    let auth_request = (Action::Read, Object::Spu, None);
    if let Ok(authorized) = auth_ctx.auth.enforce(auth_request).await {
        if !authorized {
            trace!("authorization failed");
            // If permission denied, return empty list;
            return Ok(ListResponse::Spu(vec![]));
        }
    }

    let spus: Vec<Metadata<SpuSpec>> = auth_ctx
        .global_ctx
        .spus()
        .store()
        .read()
        .await
        .values()
        .filter_map(|value| {
            if filters.filter(value.key()) {
                Some(value.inner().clone().into())
            } else {
                None
            }
        })
        .collect();

    debug!("fetched {} spu items", spus.len());
    trace!("fetch spus items detail: {:#?}", spus);

    Ok(ListResponse::Spu(spus))
}
