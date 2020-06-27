use std::io::Error;

use log::{trace, debug};

use sc_api::objects::*;
use sc_api::spu::SpuSpec;
use sc_api::spu::CustomSpuSpec;

use crate::core::SharedContext;
use crate::stores::KeyFilter;

pub async fn handle_fetch_custom_spu_request(
    filters: Vec<String>,
    ctx: SharedContext,
) -> Result<ListResponse, Error> {
    let spus: Vec<Metadata<SpuSpec>> = ctx
        .spus()
        .values()
        .filter_map(|value| {
            if value.spec().is_custom() && filters.filter(value.key()) {
                Some(value.into())
            } else {
                None
            }
        })
        .collect();

    let custom_spus: Vec<Metadata<CustomSpuSpec>> = spus.into_iter()
        .map(|spu| Metadata {
            name: spu.name,
            spec: spu.spec.into(),
            status: spu.status
        }).collect();

    debug!("flv fetch custom resp: {} items", custom_spus.len());
    trace!("flv fetch custom spus resp {:#?}", custom_spus);

    Ok(ListResponse::CustomSpu(custom_spus))
}

pub async fn handle_fetch_spus_request(
    filters: Vec<String>,
    ctx: SharedContext,
) -> Result<ListResponse, Error> {
    let spus: Vec<Metadata<SpuSpec>> = ctx
        .spus()
        .values()
        .filter_map(|value| {
            if filters.filter(value.key()) {
                Some(value.into())
            } else {
                None
            }
        })
        .collect();

    debug!("flv fetch spus resp: {} items", spus.len());
    trace!("flv fetch spus resp {:#?}", spus);

    Ok(ListResponse::Spu(spus))
}
