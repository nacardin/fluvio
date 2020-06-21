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
use crate::AdminRequest;
use super::CustomSpu;

#[derive(Encode, Decode, Default, Debug)]
pub struct UnregisterCustomSpuRequest {
       pub spu: CustomSpu,
}

impl Request for UnregisterCustomSpuRequest {
    const API_KEY: u16 = ScPublicApiKey::UnregisterCustomSpu as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}

impl AdminRequest for UnregisterCustomSpuRequest{}