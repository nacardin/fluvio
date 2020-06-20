//!
//! # Partition Status
//!
//! Interface to the Partition metadata status in K8 key value store
//!
use serde::Deserialize;
use serde::Serialize;

use k8_obj_metadata::Status;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartitionStatus {
    pub resolution: PartitionResolution,
    pub leader: ReplicaStatus,
    pub replicas: Vec<ReplicaStatus>,
    pub lsr: u32,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaStatus {
    pub spu: i32,
    pub hw: i64,
    pub leo: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PartitionResolution {
    Offline, // no leader
    Online,  // leader is available
    LeaderOffline,
    ElectionLeaderFound,
}

impl Default for PartitionResolution {
    fn default() -> Self {
        PartitionResolution::Offline
    }
}

impl Status for PartitionStatus {}



#[cfg(feature ="kf")]
mod convert {

    use k8_metadata::partition::PartitionStatus as K8PartitionStatus;
    use k8_metadata::partition::ReplicaStatus as K8ReplicaStatus;
    use k8_metadata::partition::PartitionResolution as K8PartitionResolution;


    impl From<K8PartitionStatus> for PartitionStatus {
        fn from(kv_status: K8PartitionStatus) -> Self {
            Self {
                resolution: kv_status.resolution.into(),
                leader: kv_status.leader.into(),
                replicas: kv_status
                    .replicas
                    .into_iter()
                    .map(|lrs| lrs.into())
                    .collect(),
                lsr: kv_status.lsr,
            }
        }
    }

    impl From<PartitionStatus> for K8PartitionStatus {
        fn from(status: PartitionStatus) -> K8PartitionStatus {
            K8PartitionStatus {
                resolution: status.resolution.into(),
                leader: status.leader.into(),
                replicas: status.replicas.into_iter().map(|lrs| lrs.into()).collect(),
                lsr: status.lsr.into(),
            }
        }
    }


impl From<K8PartitionResolution> for PartitionResolution {
    fn from(resolution: K8PartitionResolution) -> Self {
        match resolution {
            K8PartitionResolution::Offline => Self::Offline,
            K8PartitionResolution::Online => Self::Online,
            K8PartitionResolution::ElectionLeaderFound => Self::ElectionLeaderFound,
            K8PartitionResolution::LeaderOffline => Self::LeaderOffline,
        }
    }
}

impl From<PartitionResolution> for K8PartitionResolution {
    fn from(resolution: PartitionResolution) -> Self {
        match resolution {
            PartitionResolution::Offline => Self::Offline,
            PartitionResolution::Online => Self::Online,
            PartitionResolution::LeaderOffline => Self::LeaderOffline,
            PartitionResolution::ElectionLeaderFound => Self::ElectionLeaderFound,
        }
    }
}


impl From<K8ReplicaStatus> for ReplicaStatus {
    fn from(status: K8ReplicaStatus) -> Self {
        Self {
            spu: status.spu,
            hw: status.hw,
            leo: status.leo,
        }
    }
}

impl From<ReplicaStatus> for K8ReplicaStatus {
    fn from(status: ReplicaStatus) -> Self {
        Self {
            spu: status.spu,
            hw: status.hw,
            leo: status.leo,
        }
    }
}

}