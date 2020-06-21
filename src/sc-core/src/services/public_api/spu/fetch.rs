use std::io::Error;

use log::{trace, debug};

use kf_protocol::api::{RequestMessage, ResponseMessage};
use sc_api::spu::*;

use crate::core::SharedContext;
use crate::stores::spu::*;

pub async fn handle_fetch_spus_request(
    request: RequestMessage<FetchSpusRequest>,
    metadata: SharedContext,
) -> Result<ResponseMessage<FetchSpusResponse>, Error> {

    let (header, req) = request.get_header_request();

    // identify query type
    let (query_custom, query_type) = match req.spu_type {
        RequestSpuType::Custom => (true, "custom"),
        RequestSpuType::All => (false, "all"),
    };

    // traverse and convert spus to FLV response
    let mut flv_spu: Vec<FetchSpu> = Vec::default();
    for (name, spu) in metadata.spus().inner_store().read().iter() {
        // skip custom if necessary
        if query_custom && !spu.is_custom() {
            continue;
        }
        flv_spu.push(spu_store_metadata_to_spu_response(name, spu));
    }

    debug!(
        "flv fetch {} spus resp: {} items",
        query_type,
        flv_spu.len()
    );
    trace!("flv fetch {} spus resp {:#?}", query_type, flv_spu);

    // prepare response
    let mut response = FetchSpusResponse::default();
    response.spus = flv_spu;

    Ok(ResponseMessage::from_header(&header,response))
}

/// Encode Spus metadata into SPU FLV response
fn spu_store_metadata_to_spu_response(name: &str, spu: &SpuKV) -> FetchSpu {
    
    FetchSpu {
        name: name.to_owned(),
        spec: spu.spec.clone(),
        status: spu.status.clone()
    }
}
