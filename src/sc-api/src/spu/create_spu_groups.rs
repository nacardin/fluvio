//!
//! # Create SPU Groups
//!
//! Public API to request the SC to create managed spu groups
//!
//!
use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};
use flv_metadata::spg::*;

use crate::FlvResponseMessage;
use crate::ScPublicApiKey;

// -----------------------------------
// FlvCreateSpuGroupsRequest
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvCreateSpuGroupRequest {
    pub name: String,
    pub spec: SpuGroupSpec
}

impl Request for FlvCreateSpuGroupRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvCreateSpuGroup as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvCreateSpuGroupResponse;
}


// -----------------------------------
// FlvCreateSpuGroupsResponse
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvCreateSpuGroupResponse {
    /// The spu group creation result messages.
    pub status: FlvResponseMessage
}
