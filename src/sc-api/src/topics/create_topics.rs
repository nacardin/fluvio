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
use crate::AdminRequest;

#[derive(Encode, Decode, Default, Debug)]
pub struct CreateTopicRequest {
    pub name: String,
    pub spec: TopicSpec,
    pub dry_run: bool           
}

impl Request for CreateTopicRequest {
    const API_KEY: u16 = ScPublicApiKey::CreateTopic as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}

impl AdminRequest for CreateTopicRequest{}