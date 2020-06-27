use std::io::Error;

use log::{trace, debug};

use kf_protocol::api::{RequestMessage, ResponseMessage};
use sc_api::objects::*;

use crate::core::SharedContext;
use crate::stores::spu::*;

pub async fn handle_list_request(
    request: RequestMessage<ListRequest>,
    metadata: SharedContext,
) -> Result<ResponseMessage<ListResponse>, Error> {
    let (header, req) = request.get_header_request();

    let response = match req {
        ListRequest::Topic(filter) => {}
        ListRequest::Spu(filter) => {}
        ListRequest::SpuGroup(filter) => {}
        ListRequest::Spu(filter) => {}
    };

    Ok(ResponseMessage::from_header(&header, response))
}
