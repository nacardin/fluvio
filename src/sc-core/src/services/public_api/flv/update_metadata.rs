use std::io::Error as IoError;

use kf_protocol::api::{RequestMessage, ResponseMessage};
use kf_protocol::api::FlvErrorCode;
use sc_api::{FlvResponseMessage};
use sc_api::metadata::*;

use crate::core::spus::SpuKV;
use super::PublicContext;
pub async fn handle_metadata_update(
    request: RequestMessage<UpdateMetadataRequest>,
) -> Result<ResponseMessage<UpdateMetadataResponse>, IoError> {
}
