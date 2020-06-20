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

    use flv_metadata::spu::SpuStatus as  FlvSpuStatus;
    use flv_metadata::spu::SpuResolution as FlvSpuStatusResolution;

    use super::*;

    impl Into<FlvSpuStatus> for SpuStatus {
        fn into(self) -> FlvSpuStatus {
            FlvSpuStatus {
                resolution: self.resolution.into()
            }
        }
    }

    impl Into<FlvSpuStatusResolution> for SpuStatusResolution {
        fn into(self) -> FlvSpuStatusResolution {
            match self {
                Self::Online => FlvSpuStatusResolution::Online,
                Self::Offline => FlvSpuStatusResolution::Offline,
                Self::Init => FlvSpuStatusResolution::Init,
                
            }
        }
    }

    impl From<FlvSpuStatus> for SpuStatus {
        fn from(status: FlvSpuStatus) -> Self {
            Self {
                resolution: status.resolution.into()
            }
        }
    }

    impl From<FlvSpuStatusResolution> for SpuStatusResolution {
        fn from(resolution: FlvSpuStatusResolution) -> Self {
           match resolution {
            FlvSpuStatusResolution::Online => Self::Online,
            FlvSpuStatusResolution::Offline => Self::Offline,
            FlvSpuStatusResolution::Init => Self::Init,
            }
        }
    }
}
