//!
//! # Create Custom SPU
//!
//! Public API to request the SC to create one or more custom spus
//!
//!
use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};

use flv_metadata::spu::*;
use crate::FlvStatus;
use crate::ScPublicApiKey;

// -----------------------------------
// FlvRegisterCustomSpusRequest
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvRegisterCustomSpuRequest {

    pub name: String,
    pub spec: SpuSpec,
    pub dry_run: bool
}

impl Request for FlvRegisterCustomSpuRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvRegisterCustomSpu as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}
