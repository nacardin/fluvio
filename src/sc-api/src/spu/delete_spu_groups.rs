//!
//! # Delete Spu Groups
//!
//! Public API to request the SC to delete one or more managed spu groups.
//!
//!

use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};

use crate::FlvStatus;
use crate::ScPublicApiKey;


#[derive(Encode, Decode, Default, Debug)]
pub struct FlvDeleteSpuGroupRequest {
    pub name: String
}

impl Request for FlvDeleteSpuGroupRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvDeleteSpuGroup as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}
