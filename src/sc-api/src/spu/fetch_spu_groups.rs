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
use crate::AdminRequest;

use super::SpuGroupSpec;

/// Fetch SPU Groups by type
#[derive(Decode, Encode, Default, Debug)]
pub struct FetchSpuGroupsRequest {}

impl Request for FetchSpuGroupsRequest {
    const API_KEY: u16 = ScPublicApiKey::FetchSpuGroups as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FetchSpuGroupsResponse;
}

impl AdminRequest for FetchSpuGroupsRequest{}

#[derive(Encode, Decode, Default, Debug)]
pub struct FetchSpuGroupsResponse {
    pub error: FlvStatus,
    /// Each spu in the response.
    pub spu_groups: Vec<FetchSpuGroup>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct FetchSpuGroup {
    pub name: String,
    pub spec: SpuGroupSpec,
    pub status: SpuGroupStatus
}

