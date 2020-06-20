//!
//! # Partition Spec
//!
//! Interface to the Partition metadata spec in K8 key value store
//!
use k8_obj_metadata::Crd;
use k8_obj_metadata::Spec;
use k8_obj_metadata::DefaultHeader;

use serde::Deserialize;
use serde::Serialize;

use super::PartitionStatus;
use super::PARTITION_API;

impl Spec for PartitionSpec {
    type Header = DefaultHeader;
    type Status = PartitionStatus;
    fn metadata() -> &'static Crd {
        &PARTITION_API
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartitionSpec {
    pub leader: i32,
    pub replicas: Vec<i32>,
}

impl PartitionSpec {
    pub fn new(leader: i32, replicas: Vec<i32>) -> Self {
        PartitionSpec { leader, replicas }
    }
}


#[cfg(feature ="kf")]
mod convert {

    use std::convert::Into;

    use flv_metadata::partition::PartitionSpec as KFPartitionSpec;
    use super::*;

    impl Into<KFPartitionSpec> for PartitionSpec {
        fn from(kv_spec: PartitionSpec) -> Self {
            KFPartitionSpec {
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


}