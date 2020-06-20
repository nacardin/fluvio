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

    use std::convert::Into;

    use flv_metadata::partition::PartitionStatus as KfPartitionStatus;
    use flv_metadata::partition::ReplicaStatus as KfReplicaStatus;
    use flv_metadata::partition::PartitionResolution as KfPartitionResolution;

    use super::*;

    impl Into<KfPartitionStatus> for PartitionStatus {
        fn into(self) -> KfPartitionStatus {
            KfPartitionStatus {
                resolution: self.resolution.into(),
                leader: self.leader.into(),
                replicas: self
                    .replicas
                    .into_iter()
                    .map(|lrs| lrs.into())
                    .collect(),
                lsr: self.lsr,
            }
        }
    }

    impl From<KfPartitionStatus> for PartitionStatus {
        fn from(status: KfPartitionStatus) -> Self {
            Self {
                resolution: status.resolution.into(),
                leader: status.leader.into(),
                replicas: status.replicas.into_iter().map(|lrs| lrs.into()).collect(),
                lsr: status.lsr.into(),
            }
        }
    }


    impl Into<KfPartitionResolution> for PartitionResolution {
        fn into(self) -> KfPartitionResolution {
            match self {
                Self::Offline => KfPartitionResolution::Offline,
                Self::Online => KfPartitionResolution::Online,
                Self::ElectionLeaderFound => KfPartitionResolution::ElectionLeaderFound,
                Self::LeaderOffline => KfPartitionResolution::LeaderOffline,
            }
        }
    }

    impl From<KfPartitionResolution> for PartitionResolution {
        fn from(resolution: KfPartitionResolution) -> Self {
            match resolution {
                KfPartitionResolution::Offline => Self::Offline,
                KfPartitionResolution::Online => Self::Online,
                KfPartitionResolution::LeaderOffline => Self::LeaderOffline,
                KfPartitionResolution::ElectionLeaderFound => Self::ElectionLeaderFound,
            }
        }
    }


    impl Into<KfReplicaStatus> for ReplicaStatus {
        fn into(self) -> KfReplicaStatus {
            KfReplicaStatus {
                spu: self.spu,
                hw: self.hw,
                leo: self.leo,
            }
        }
    }

    impl From<KfReplicaStatus> for ReplicaStatus {
        fn from(status: KfReplicaStatus) -> Self {
            Self {
                spu: status.spu,
                hw: status.hw,
                leo: status.leo,
            }
        }
    }

}