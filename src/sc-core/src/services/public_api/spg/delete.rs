//!
//! # Delete Spu Groups Request
//!
//! Delete spu groups request handler. Lookup spu-group in local metadata, grab its K8 context
//! and send K8 a delete message.
//!
use log::{debug, trace};
use std::io::Error;


use k8_metadata_client::MetadataClient;
use kf_protocol::api::FlvErrorCode;
use kf_protocol::api::{RequestMessage, ResponseMessage};
use sc_api::FlvStatus;
use sc_api::spu::DeleteSpuGroupRequest;

use super::PublicContext;

/// Handler for delete spu group request
pub async fn handle_delete_spu_group_request<C>(
    request: RequestMessage<DeleteSpuGroupRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, Error>
where
    C: MetadataClient,
{
    use flv_metadata::spg::K8SpuGroupSpec;

    let (header, req) = request.get_header_request();

    let name = req.name;
    debug!("delete spg group: {}",name);

    let status = match ctx.delete::<K8SpuGroupSpec>(&name).await {
        Ok(_) => FlvStatus::new_ok(name),
        Err(err) => {
              let error = Some(err.to_string());
              FlvStatus::new(name, FlvErrorCode::SpuError, error)
        }
    };

    trace!("flv delete spu group resp {:#?}", status);   

    Ok(ResponseMessage::from_header(&header,status))
}


