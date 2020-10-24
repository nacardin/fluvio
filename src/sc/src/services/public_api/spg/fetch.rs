use std::io::{Error, ErrorKind};

use tracing::debug;
use tracing::trace;


use fluvio_sc_schema::objects::{ ListResponse, NameFilter, Metadata };
use fluvio_sc_schema::spg::SpuGroupSpec;

use crate::services::auth::AuthServiceContext;

pub async fn handle_fetch_spu_groups_request(
    filters: Vec<NameFilter>,
    auth_ctx: &AuthServiceContext,
) -> Result<ListResponse, Error> {
    debug!("fetching spu groups");

    if let Ok(authorized) = auth_ctx.auth.read::<SpuGroupSpec>().await {
        if !authorized {
            trace!("authorization failed");
            // If permission denied, return empty list;
            return Ok(ListResponse::SpuGroup(vec![]));
        }
    } else {
        return Err(Error::new(ErrorKind::Interrupted, "authorization io error"));
    }

    let spgs: Vec<Metadata<SpuGroupSpec>> = auth_ctx
        .global_ctx
        .spgs()
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

    debug!("flv fetch spgs resp: {} items", spgs.len());
    trace!("flv fetch spgs resp {:#?}", spgs);

    Ok(ListResponse::SpuGroup(spgs))
}
