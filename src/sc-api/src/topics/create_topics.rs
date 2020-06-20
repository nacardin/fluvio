//!
//! # Create Topics
//!
//! Public API to request the SC to create one or more topics.
//!
//!

use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};

use flv_metadata::topic::*;

use crate::FlvStatus;
use crate::ScPublicApiKey;

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvCreateTopicRequest {
    pub name: String,
    pub spec: TopicSpec,
    pub dry_run: bool           
}

impl Request for FlvCreateTopicRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvCreateTopic as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}