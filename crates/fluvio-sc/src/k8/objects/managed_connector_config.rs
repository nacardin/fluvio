use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::dispatcher::core::Spec;
use crate::dispatcher::core::Status;
use crate::stores::managed_connector::ManagedConnectorSpec;

pub use k8_types::core::config_map::ConfigMapSpec as K8ConfigMapSpec;
pub use k8_types::core::config_map::ConfigMapStatus as K8ConfigMapStatus;

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
// #[serde(transparent)]
// pub struct ManagedConnectorConfigMapSpec(K8ConfigMapSpec);
pub struct ManagedConnectorConfigMapSpec;

impl Spec for ManagedConnectorConfigMapSpec {
    const LABEL: &'static str = "ConfigMap";
    type IndexKey = String;
    type Status = ConfigMapStatus;
    type Owner = ManagedConnectorSpec;
}

impl From<K8ConfigMapSpec> for ManagedConnectorConfigMapSpec {
    fn from(k8: K8ConfigMapSpec) -> Self {
        // Self(k8)
        Self
    }
}

impl From<ManagedConnectorConfigMapSpec> for K8ConfigMapSpec {
    fn from(spec: ManagedConnectorConfigMapSpec) -> Self {
        // spec.0
        K8ConfigMapSpec {  }
    }
}

/// Statefulset Spec
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
// #[serde(transparent)]
// pub struct ConfigMapStatus(pub K8ConfigMapStatus);
pub struct ConfigMapStatus;

impl Status for ConfigMapStatus {}

impl fmt::Display for ConfigMapStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:#?}", self.0)
        write!(f, "{:#?}", self)
    }
}

impl From<K8ConfigMapStatus> for ConfigMapStatus {
    fn from(k8: K8ConfigMapStatus) -> Self {
        // Self(k8)
        Self
    }
}

impl From<ConfigMapStatus> for K8ConfigMapStatus {
    fn from(status: ConfigMapStatus) -> Self {
        // status.0
        Self {}
    }
}

mod extended {

    use k8_types::K8Obj;

    use crate::stores::k8::K8ConvertError;
    use crate::stores::k8::K8ExtendedSpec;
    use crate::stores::k8::K8MetaItem;
    use crate::stores::MetadataStoreObject;
    use crate::stores::k8::default_convert_from_k8;

    use super::*;

    impl K8ExtendedSpec for ManagedConnectorConfigMapSpec {
        type K8Spec = K8ConfigMapSpec;
        type K8Status = K8ConfigMapStatus;

        fn convert_from_k8(
            k8_obj: K8Obj<Self::K8Spec>,
        ) -> Result<MetadataStoreObject<Self, K8MetaItem>, K8ConvertError<Self::K8Spec>> {
            default_convert_from_k8(k8_obj)
        }
    }
}
