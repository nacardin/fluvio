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

    use flv_metadata::spg::SpuGroupStatus as FlvSpuGroupStatus;
    use flv_metadata::spg::SpuGroupStatusResolution as FlvSpgStatusResolution;
    use super::*;

    impl Into<FlvSpuGroupStatus> for SpuGroupStatus {
        fn into(self) -> FlvSpuGroupStatus {
            FlvSpuGroupStatus {
                resolution: self.resolution.into(),
                reason: self.reason.into()
            }
        }
    }

    impl From<FlvSpuGroupStatus> for SpuGroupStatus {
        fn from(status: FlvSpuGroupStatus) -> Self {
            Self {
                resolution: status.resolution.into(),
                reason: status.reason.into()
            }
        }
    }

    impl Into<FlvSpgStatusResolution> for SpuGroupStatusResolution {
        fn into(self) -> FlvSpgStatusResolution {
            match self {
                Self::Init => FlvSpgStatusResolution::Init,
                Self::Invalid => FlvSpgStatusResolution::Invalid,
                Self::Reserved => FlvSpgStatusResolution::Reserved,
            }
        }
    }

    impl From<FlvSpgStatusResolution> for SpuGroupStatusResolution {
        fn from(status: FlvSpgStatusResolution) -> Self {
            match status {
                FlvSpgStatusResolution::Init => Self::Init,
                FlvSpgStatusResolution::Invalid => Self::Invalid,
                FlvSpgStatusResolution::Reserved => Self::Reserved,
            }
        }
    }


}