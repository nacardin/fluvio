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
use crate::AdminRequest;

#[derive(Encode, Decode, Default, Debug)]
pub struct DeleteSpuGroupRequest {
    pub name: String
}

impl Request for DeleteSpuGroupRequest {
    const API_KEY: u16 = ScPublicApiKey::DeleteSpuGroup as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}

impl AdminRequest for DeleteSpuGroupRequest{}