//!
//! # Fetch SPUs
//!
//! Public API to fetch SPU metadata from the SC
//!
use kf_protocol::api::Request;
use kf_protocol::derive::Decode;
use kf_protocol::derive::Encode;

use flv_metadata::spu::*;

use crate::ScPublicApiKey;


// -----------------------------------
// FlvFetchSpusRequest
// -----------------------------------

/// Fetch SPUs by type
#[derive(Decode, Encode, Default, Debug)]
pub struct FlvFetchSpusRequest {
    /// SPU type All or Custom
    pub spu_type: FlvRequestSpuType,
}

impl Request for FlvFetchSpusRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvFetchSpus as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvFetchSpusResponse;
}

#[derive(Decode, Encode, Debug)]
pub enum FlvRequestSpuType {
    All,
    Custom,
}

impl Default for FlvRequestSpuType {
    fn default() -> FlvRequestSpuType {
        FlvRequestSpuType::All
    }
}


// -----------------------------------
// FlvFetchSpusResponse
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvFetchSpusResponse {
    /// Each spu in the response.
    pub spus: Vec<FlvFetchSpu>,
}


#[derive(Encode, Decode, Default, Debug)]
pub struct FlvFetchSpu {

    pub name: String,

    pub spec: SpuSpec,

    pub status: SpuStatus,
}
