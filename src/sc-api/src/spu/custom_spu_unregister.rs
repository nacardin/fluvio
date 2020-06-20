//!
//! # Delete Custon SPUs
//!
//! Public API to request the SC to delete one or more custom spus.
//!
//!
use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};

use crate::FlvStatus;
use crate::ScPublicApiKey;
use super::FlvCustomSpu;

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvUnregisterCustomSpuRequest {
   
    pub spu: FlvCustomSpu,
}

impl Request for FlvUnregisterCustomSpuRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvUnregisterCustomSpu as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}
