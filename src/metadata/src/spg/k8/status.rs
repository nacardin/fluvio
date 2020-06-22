//!
//! # Cluster Status
//!
//! Interface to the Cluster metadata status in K8 key value store
//!
use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use k8_obj_metadata::Status;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct K8SpuGroupStatus {
    pub resolution: K8SpuGroupStatusResolution,
    pub reason: Option<String>,
}

impl Status for K8SpuGroupStatus {}

impl K8SpuGroupStatus {
    pub fn invalid(reason: String) -> Self {
        Self {
            resolution: K8SpuGroupStatusResolution::Invalid,
            reason: Some(reason),
        }
    }

    pub fn reserved() -> Self {
        Self {
            resolution: K8SpuGroupStatusResolution::Reserved,
            ..Default::default()
        }
    }

    pub fn is_already_valid(&self) -> bool {
        self.resolution == K8SpuGroupStatusResolution::Reserved
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum K8SpuGroupStatusResolution {
    Init,
    Invalid,
    Reserved,
}

impl Default for K8SpuGroupStatusResolution {
    fn default() -> Self {
        Self::Init
    }
}

impl fmt::Display for K8SpuGroupStatusResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Init => write!(f, "Init"),
            Self::Invalid => write!(f, "Invalid"),
            Self::Reserved => write!(f, "Reserved"),
        }
    }
}



mod convert {

    use crate::spg::SpuGroupStatus;
    use crate::spg::SpuGroupStatusResolution;
    use super::*;

    impl From<K8SpuGroupStatus> for SpuGroupStatus {
        fn from(status: K8SpuGroupStatus) -> Self {
            Self {
                resolution: status.resolution.into(),
                reason: status.reason.into()
            }
        }
    }

    impl From<SpuGroupStatus> for K8SpuGroupStatus {
        fn from(status: SpuGroupStatus) -> Self {
            Self {
                resolution: status.resolution.into(),
                reason: status.reason.into()
            }
        }
    }

    impl From<K8SpuGroupStatusResolution> for SpuGroupStatusResolution {
        fn from(resolution: K8SpuGroupStatusResolution) -> Self {
            match resolution {
                K8SpuGroupStatusResolution::Init => Self::Init,
                K8SpuGroupStatusResolution::Invalid => Self::Invalid,
                K8SpuGroupStatusResolution::Reserved => Self::Reserved,
            }
        }
    }

    impl From<SpuGroupStatusResolution> for K8SpuGroupStatusResolution {
        fn from(status: SpuGroupStatusResolution) -> Self {
            match status {
                SpuGroupStatusResolution::Init => Self::Init,
                SpuGroupStatusResolution::Invalid => Self::Invalid,
                SpuGroupStatusResolution::Reserved => Self::Reserved,
            }
        }
    }


}