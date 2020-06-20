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
use crate::ApiError;

// -----------------------------------
// FlvCreateSpuGroupsRequest
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvCreateSpuGroupsRequest {
    /// A list of one or more spu groups to be created.
    pub spu_groups: Vec<SpuGroupSpec>,
}

impl Request for FlvCreateSpuGroupsRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvCreateSpuGroups as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvCreateSpuGroupsResponse;
}

// quick way to convert a single group into groups requests
impl From<SpuGroupSpec> for FlvCreateSpuGroupsRequest {
    fn from(group: SpuGroupSpec) -> Self {
        let mut groups = Self::default();
        groups.spu_groups.push(group);
        groups
    }
}


// -----------------------------------
// FlvCreateSpuGroupsResponse
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvCreateSpuGroupsResponse {
    /// The spu group creation result messages.
    pub results: Vec<FlvResponseMessage>,
}

impl FlvCreateSpuGroupsResponse {
    /// validate and extract a single response
    pub fn validate(self) -> Result<(), ApiError> {
        // ? what is name, so just find first item
        if let Some(item) = self.results.into_iter().find(|_| true) {
            item.as_result()
        } else {
            Err(ApiError::NoResourceFounded("custom spu".to_owned()))
        }
    }
}
