//!
//! # Topic Status
//!
//! Interface to the Topic metadata status in K8 key value store
//!
use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use k8_obj_metadata::Status;

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopicStatus {
    pub resolution: TopicStatusResolution,
    pub replica_map: BTreeMap<i32, Vec<i32>>,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum TopicStatusResolution {
    Init,                  // initial state
    Pending,               // waiting for resources (spus)
    InsufficientResources, // out of sync with partition/replication_factor
    InvalidConfig,         // invalid configuration
    Provisioned,           // spu allocated
}

impl Default for TopicStatusResolution {
    fn default() -> Self {
        TopicStatusResolution::Init
    }
}

impl Status for TopicStatus {}

#[cfg(feature = "flv")]
mod convert {

    use k8_metadata::topic::TopicStatus as K8TopicStatus;
    use k8_metadata::topic::TopicStatusResolution as K8TopicStatusResolution;

    impl From<K8TopicStatus> for TopicStatus {
        fn from(kv_status: K8TopicStatus) -> Self {
            let resolution = match kv_status.resolution {
                K8TopicStatusResolution::Provisioned => TopicResolution::Provisioned,
                K8TopicStatusResolution::Init => TopicResolution::Init,
                K8TopicStatusResolution::Pending => TopicResolution::Pending,
                K8TopicStatusResolution::InsufficientResources => {
                    TopicResolution::InsufficientResources
                }
                K8TopicStatusResolution::InvalidConfig => TopicResolution::InvalidConfig,
            };

            TopicStatus {
                resolution,
                replica_map: kv_status.replica_map.clone(),
                reason: kv_status.reason.clone(),
            }
        }
    }

    impl From<TopicStatus> for K8TopicStatus {
        fn from(status: TopicStatus) -> K8TopicStatus {
            let resolution = match status.resolution {
                TopicResolution::Provisioned => K8TopicStatusResolution::Provisioned,
                TopicResolution::Init => K8TopicStatusResolution::Init,
                TopicResolution::Pending => K8TopicStatusResolution::Pending,
                TopicResolution::InsufficientResources => {
                    K8TopicStatusResolution::InsufficientResources
                }
                TopicResolution::InvalidConfig => K8TopicStatusResolution::InvalidConfig,
            };

            K8TopicStatus {
                resolution: resolution,
                replica_map: status.replica_map.clone(),
                reason: status.reason.clone(),
            }
        }
    }
}
