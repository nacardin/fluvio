use kf_protocol::derive::Decode;
use kf_protocol::derive::Encode;
use flv_metadata::api::Message;


use super::replica::ReplicaLeader;

pub type ReplicaMsg = Message<ReplicaLeader>;

/// Changes in the Replica Specs
#[derive(Decode, Encode, Debug, Default)]
pub struct UpdateReplicaResponse {
    pub replicas: Vec<ReplicaMsg>,
}

impl UpdateReplicaResponse {
    pub fn new(replica_msgs: Vec<ReplicaMsg>) -> Self {
        Self {
            replicas: replica_msgs,
        }
    }
}
