//!
//! # Delete Topics
//!
//! Public API to request the SC to delete one or more topics.
//!
//!

use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};

use crate::FlvStatus;
use crate::ScPublicApiKey;

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvDeleteTopicRequest {
    pub name: String
}

impl Request for FlvDeleteTopicRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvDeleteTopic as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}
