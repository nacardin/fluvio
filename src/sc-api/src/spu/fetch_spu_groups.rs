//!
//! # Fetch SPU Groups
//!
//! Public API to fetch SPU Group metadata from the SC
//!
use kf_protocol::api::Request;
use kf_protocol::derive::Decode;
use kf_protocol::derive::Encode;
use flv_metadata::spg::*;


use crate::ScPublicApiKey;
use crate::FlvStatus;

use super::SpuGroupSpec;

/// Fetch SPU Groups by type
#[derive(Decode, Encode, Default, Debug)]
pub struct FlvFetchSpuGroupsRequest {}

impl Request for FlvFetchSpuGroupsRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvFetchSpuGroups as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvFetchSpuGroupsResponse;
}

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvFetchSpuGroupsResponse {
    pub error: FlvStatus,
    /// Each spu in the response.
    pub spu_groups: Vec<FlvFetchSpuGroup>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvFetchSpuGroup {
    spec: SpuGroupSpec,
    status: SpuGroupResponse
}

