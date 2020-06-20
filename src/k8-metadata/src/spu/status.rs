//!
//! # SPU Status
//!
//! Interface to the SPU metadata status in K8 key value store
//!
use serde::Deserialize;
use serde::Serialize;

use k8_obj_metadata::Status;

// -----------------------------------
// Data Structures
// -----------------------------------

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct SpuStatus {
    pub resolution: SpuStatusResolution,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum SpuStatusResolution {
    Online,
    Offline,
    Init,
}

impl Default for SpuStatusResolution {
    fn default() -> Self {
        SpuStatusResolution::Init
    }
}

impl Status for SpuStatus {}

#[cfg(feature = "flv")]
mod convert {

    use k8_metadata::spu::SpuStatus as K8SpuStatus;
    use k8_metadata::spu::SpuStatusResolution as K8SpuStatusResolution;

    impl From<K8SpuStatus> for SpuStatus {
        fn from(kv_status: K8SpuStatus) -> Self {
            SpuStatus {
                resolution: match kv_status.resolution {
                    K8SpuStatusResolution::Online => SpuResolution::Online,
                    K8SpuStatusResolution::Offline => SpuResolution::Offline,
                    K8SpuStatusResolution::Init => SpuResolution::Init,
                },
            }
        }
    }

    impl From<SpuStatus> for K8SpuStatus {
        fn from(status: SpuStatus) -> K8SpuStatus {
            K8SpuStatus {
                resolution: (match status.resolution {
                    SpuResolution::Online => K8SpuStatusResolution::Online,
                    SpuResolution::Offline => K8SpuStatusResolution::Offline,
                    SpuResolution::Init => K8SpuStatusResolution::Init,
                }),
            }
        }
    }
}
