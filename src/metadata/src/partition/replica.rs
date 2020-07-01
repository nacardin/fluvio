use std::fmt;

use kf_protocol::derive::{Decode, Encode};
use flv_types::SpuId;
use crate::partition::ReplicaKey;

#[derive(Decode, Encode, Debug, PartialEq, Clone, Default)]
pub struct Replica {
    pub id: ReplicaKey,
    pub leader: SpuId,
    pub replicas: Vec<SpuId>,
}

impl fmt::Display for Replica {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} leader: {} replicas: [", self.id, self.leader)?;
        for replica in &self.replicas {
            write!(f, "{},", replica)?;
        }
        write!(f, "]")
    }
}



/// given replica, where is leader
#[derive(Decode, Encode, Debug, PartialEq, Clone, Default)]
pub struct ReplicaLeader {
    pub id: ReplicaKey,
    pub leader: SpuId,
}
