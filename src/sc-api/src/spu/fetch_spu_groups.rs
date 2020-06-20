//!
//! # Fetch SPU Groups
//!
//! Public API to fetch SPU Group metadata from the SC
//!
use kf_protocol::api::Request;
use kf_protocol::derive::Decode;
use kf_protocol::derive::Encode;


use crate::ScPublicApiKey;
use crate::FlvResponseMessage;

use super::FlvSpuGroupResolution;

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
    pub error: FlvResponseMessage,
    /// Each spu in the response.
    pub spu_groups: Vec<FlvFetchSpuGroup>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvFetchSpuGroup {
    pub name: String,

    /// The number of replicas for the spu group
    pub replicas: u16,

    // The base spu id for the spu group
    pub min_id: i32,

    /// Rack label, optional parameter used by replica assignment algorithm.
    pub rack: Option<String>,

    /// storage size
    pub size: String,

    /// Status resolution
    pub resolution: FlvSpuGroupResolution,

    /// Reason for Status resolution (if applies)
    pub reason: Option<String>,
}
