//!
//! # Partition Spec
//!
//!
use flv_types::SpuId;
use kf_protocol::derive::{Decode, Encode};
use k8_metadata::partition::PartitionSpec as K8PartitionSpec;

/// Spec for Partition
/// Each partition has replicas spread among SPU
/// one of replica is leader which is duplicated in the leader field
#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub struct PartitionSpec {
    pub leader: SpuId,
    pub replicas: Vec<SpuId>,
}


impl From<K8PartitionSpec> for PartitionSpec {
    fn from(kv_spec: K8PartitionSpec) -> Self {
        PartitionSpec {
            leader: kv_spec.leader,
            replicas: kv_spec.replicas,
        }
    }
}

impl From<PartitionSpec> for K8PartitionSpec {
    fn from(spec: PartitionSpec) -> K8PartitionSpec {
        K8PartitionSpec {
            leader: spec.leader,
            replicas: spec.replicas,
        }
    }
}


impl std::default::Default for PartitionSpec {
    fn default() -> Self {
        PartitionSpec {
            leader: 0,
            replicas: Vec::default(),
        }
    }
}



impl PartitionSpec {
    pub fn new(leader: SpuId, replicas: Vec<SpuId>) -> Self {
        Self { leader, replicas }
    }

    pub fn has_spu(&self, spu: &SpuId) -> bool {
        self.replicas.contains(spu)
    }
}

impl From<Vec<i32>> for PartitionSpec {
    fn from(replicas: Vec<i32>) -> Self {
        if replicas.len() > 0 {
            Self::new(replicas[0].clone(), replicas)
        } else {
            Self::new(0, replicas)
        }
    }
}
