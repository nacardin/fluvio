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
use crate::AdminRequest;

#[derive(Encode, Decode, Default, Debug)]
pub struct DeleteTopicRequest {
    pub name: String
}

impl Request for DeleteTopicRequest {
    const API_KEY: u16 = ScPublicApiKey::DeleteTopic as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}


impl AdminRequest for DeleteTopicRequest{}