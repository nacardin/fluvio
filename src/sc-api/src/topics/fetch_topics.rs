//!
//! # Fetch Topics
//!
//! Public API to retrieve Topics from the SC.
//!
use kf_protocol::api::Request;
use kf_protocol::api::FlvErrorCode;
use kf_protocol::derive::Decode;
use kf_protocol::derive::Encode;

use flv_metadata::topic::{TopicSpec, TopicStatus};

use crate::ScPublicApiKey;
use crate::AdminRequest;

// -----------------------------------
// FlvFetchTopicsRequest
// -----------------------------------

#[derive(Decode, Encode, Default, Debug)]
pub struct FetchTopicsRequest {
    /// A list of one or more topics to be retrieved.
    /// None retrieves all topics.
    pub names: Option<Vec<String>>,
}

impl Request for FetchTopicsRequest {
    const API_KEY: u16 = ScPublicApiKey::FetchTopics as u16;
    type Response = FetchTopicsResponse;
}

impl AdminRequest for FetchTopicsRequest{}


#[derive(Encode, Decode, Default, Debug)]
pub struct FetchTopicsResponse {
    /// The list of topics that have been retrieved.
    pub topics: Vec<FetchTopicResponse>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct FetchTopicResponse {
    /// The error code, None for no errors
    pub error_code: FlvErrorCode,

    /// The name of the topic.
    pub name: String,

    /// Topic parameters, None if error
    pub topic: Option<FetchTopic>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct FetchTopic {
    /// Topic spec
    pub spec: TopicSpec,

    /// Topic status
    pub status: TopicStatus,

    /// Replica assignment for each partition
    pub partition_replicas: Option<Vec<PartitionReplica>>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct PartitionReplica {
    /// Partition id
    pub id: i32,

    /// Replica leader
    pub leader: i32,

    /// Replica assignment
    pub replicas: Vec<i32>,

    /// Only live replicas in replica assignment
    pub live_replicas: Vec<i32>,
}


impl FetchTopicResponse {
    /// Constructor for topics found
    pub fn new(
        name: String,
        spec: TopicSpec,
        status: TopicStatus,
        partition_replicas: Option<Vec<PartitionReplica>>,
    ) -> Self {
        FetchTopicResponse {
            name: name,
            error_code: FlvErrorCode::None,
            topic: Some(FetchTopic {
                spec,
                status,
                partition_replicas,
            }),
        }
    }

    /// Constructor for topics that are not found
    pub fn new_not_found(name: String) -> Self {
        FetchTopicResponse {
            name: name,
            error_code: FlvErrorCode::TopicNotFound,
            topic: None,
        }
    }

    /// Update topic partitions.
    /// Requirements:
    ///  * Must be called with valid topic, otherwise, update will fail silently
    pub fn update_partitions(&mut self, partition_replicas: Option<Vec<PartitionReplica>>) {
        if self.topic.is_some() {
            self.topic.as_mut().unwrap().partition_replicas = partition_replicas;
        }
    }
}
