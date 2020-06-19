use kf_protocol::derive::Decode;
use kf_protocol::derive::Encode;
use flv_metadata::api::*;


use super::replica::ReplicaLeader;

pub type ReplicaMsg = Message<ReplicaLeader>;

/// Changes in the Replica Specs
#[derive(Decode, Encode, Debug, Clone, Default)]
pub struct UpdateReplicaResponse {
    pub replicas: ReplicaMsgs,
}

impl UpdateReplicaResponse {
    pub fn new(replica_msgs: ReplicaMsgs) -> Self {
        Self {
            replicas: replica_msgs,
        }
    }
}
