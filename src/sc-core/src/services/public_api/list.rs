use std::io::Error;

use log::{trace, debug};

use kf_protocol::api::{RequestMessage, ResponseMessage};
use sc_api::objects::*;

use crate::core::SharedContext;
use crate::stores::spu::*;

pub async fn handle_list_request(
    request: RequestMessage<ListRequest>,
    ctx: SharedContext,
) -> Result<ResponseMessage<ListResponse>, Error> {
    let (header, req) = request.get_header_request();

    let response = match req {
        ListRequest::Topic(filter) => {
            super::topic::handle_fetch_topics_request(filter, ctx).await?
        }
        ListRequest::Spu(filter) => super::spu::handle_fetch_spus_request(filter, ctx).await?,
        ListRequest::SpuGroup(filter) => super::spg::handle_fetch_spu_groups_request(filter, ctx).await?,
        ListRequest::CustomSpu(filter) => super::spu::handle_fetch_spus_request(filter, ctx).await?
    };

    Ok(ResponseMessage::from_header(&header, response))
}
