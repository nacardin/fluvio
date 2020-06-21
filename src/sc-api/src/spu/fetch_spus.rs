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
use crate::AdminRequest;

/// Fetch SPUs by type
#[derive(Decode, Encode, Default, Debug)]
pub struct FetchSpusRequest {
    /// SPU type All or Custom
    pub spu_type: RequestSpuType,
}

impl Request for FetchSpusRequest {
    const API_KEY: u16 = ScPublicApiKey::FetchSpus as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FetchSpusResponse;
}

impl AdminRequest for FetchSpusRequest{}

#[derive(Decode, Encode, Debug)]
pub enum RequestSpuType {
    All,
    Custom,
}

impl Default for RequestSpuType {
    fn default() -> Self {
        Self::All
    }
}


// -----------------------------------
// FlvFetchSpusResponse
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FetchSpusResponse {
    /// Each spu in the response.
    pub spus: Vec<FetchSpu>,
}


#[derive(Encode, Decode, Default, Debug)]
pub struct FetchSpu {

    pub name: String,

    pub spec: SpuSpec,

    pub status: SpuStatus,
}
