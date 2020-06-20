//!
//! # SPU Spec
//!
//! Interface to the SPU metadata spec in K8 key value store
//!
use k8_obj_metadata::Crd;
use k8_obj_metadata::Spec;
use k8_obj_metadata::DefaultHeader;
use k8_obj_core::pod::ContainerPortSpec;
use k8_obj_core::service::ServicePort;

use serde::Deserialize;
use serde::Serialize;
use super::SPU_API;
use super::SpuStatus;

impl Spec for SpuSpec {
    type Status = SpuStatus;
    type Header = DefaultHeader;

    fn metadata() -> &'static Crd {
        &SPU_API
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpuSpec {
    pub spu_id: i32,
    pub public_endpoint: IngressPort,
    pub private_endpoint: Endpoint,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spu_type: Option<SpuType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum SpuType {
    Managed,
    Custom,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct IngressPort {
    pub port: u16,
    pub ingress: Vec<IngressAddr>,
    pub encryption: EncryptionEnum,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct IngressAddr {
    pub hostname: Option<String>,
    pub ip: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub port: u16,
    pub host: String,
    pub encryption: EncryptionEnum,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum EncryptionEnum {
    PLAINTEXT,
    SSL,
}

// -----------------------------------
// Implementation - Endpoint
// -----------------------------------

impl Endpoint {
    pub fn new(port: u16, host: String) -> Self {
        Endpoint {
            port,
            host,
            encryption: EncryptionEnum::PLAINTEXT,
        }
    }
}

impl From<&Endpoint> for ContainerPortSpec {
    fn from(end_point: &Endpoint) -> Self {
        ContainerPortSpec {
            container_port: end_point.port,
            ..Default::default()
        }
    }
}

impl From<&Endpoint> for ServicePort {
    fn from(end_point: &Endpoint) -> Self {
        ServicePort {
            port: end_point.port,
            ..Default::default()
        }
    }
}

// -----------------------------------
// Implementation - EncryptionEnum
// -----------------------------------
impl Default for EncryptionEnum {
    fn default() -> EncryptionEnum {
        EncryptionEnum::PLAINTEXT
    }
}

#[cfg(feature = "flv")]
mod convert {

    use k8_metadata::spu::SpuSpec as K8SpuSpec;
    use k8_metadata::spu::SpuType as K8SpuType;
    use k8_metadata::spu::EncryptionEnum as K8EncryptionEnum;
    use k8_metadata::spu::Endpoint as K8Endpoint;
    use k8_metadata::spu::IngressPort as K8IngressPort;
    use k8_metadata::spu::IngressAddr as K8IngressAddr;
    use super::*;

    impl From<K8SpuSpec> for SpuSpec {
        fn from(kv_spec: K8SpuSpec) -> Self {
            // convert spu-type, defaults to Custom for none
            let spu_type = if let Some(kv_spu_type) = kv_spec.spu_type {
                kv_spu_type.into()
            } else {
                SpuType::Custom
            };

            // spu spec
            SpuSpec {
                id: kv_spec.spu_id,
                spu_type: spu_type,
                public_endpoint: kv_spec.public_endpoint.into(),
                private_endpoint: kv_spec.private_endpoint.into(),
                rack: kv_spec.rack.clone(),
            }
        }
    }

    impl Into<K8SpuSpec> for SpuSpec {
        fn into(self) -> K8SpuSpec {
            K8SpuSpec {
                spu_id: self.id,
                spu_type: Some(self.spu_type.into()),
                public_endpoint: self.public_endpoint.into(),
                private_endpoint: self.private_endpoint.into(),
                rack: self.rack,
            }
        }
    }

    impl From<K8IngressPort> for IngressPort {
        fn from(ingress_port: K8IngressPort) -> Self {
            Self {
                port: ingress_port.port,
                ingress: ingress_port.ingress.into_iter().map(|a| a.into()).collect(),
                encryption: ingress_port.encryption.into(),
            }
        }
    }

    impl Into<K8IngressPort> for IngressPort {
        fn into(self) -> K8IngressPort {
            K8IngressPort {
                port: self.port,
                ingress: self.ingress.into_iter().map(|a| a.into()).collect(),
                encryption: self.encryption.into(),
            }
        }
    }

    impl From<K8IngressAddr> for IngressAddr {
        fn from(addr: K8IngressAddr) -> Self {
            Self {
                hostname: addr.hostname,
                ip: addr.ip,
            }
        }
    }

    impl Into<K8IngressAddr> for IngressAddr {
        fn into(self) -> K8IngressAddr {
            K8IngressAddr {
                hostname: self.hostname,
                ip: self.ip,
            }
        }
    }

    impl From<K8Endpoint> for Endpoint {
        fn from(pt: K8Endpoint) -> Self {
            Self {
                port: pt.port,
                host: pt.host,
                encryption: pt.encryption.into(),
            }
        }
    }

    impl Into<K8Endpoint> for Endpoint {
        fn into(self) -> K8Endpoint {
            K8Endpoint {
                port: self.port,
                host: self.host,
                encryption: self.encryption.into(),
            }
        }
    }

    // EndPoint
    pub fn new(ep: K8Endpoint) -> Self {
        Self {
            port: ep.port,
            host: ep.host,
            encryption: match ep.encryption {
                K8EncryptionEnum::PLAINTEXT => EncryptionEnum::PLAINTEXT,
                K8EncryptionEnum::SSL => EncryptionEnum::SSL,
            },
        }
    }

    impl From<K8EncryptionEnum> for EncryptionEnum {
        fn from(enc: K8EncryptionEnum) -> Self {
            match enc {
                K8EncryptionEnum::PLAINTEXT => Self::PLAINTEXT,
                K8EncryptionEnum::SSL => Self::SSL,
            }
        }
    }

    impl Into<K8EncryptionEnum> for EncryptionEnum {
        fn into(self) -> K8EncryptionEnum {
            match self {
                Self::PLAINTEXT => K8EncryptionEnum::PLAINTEXT,
                Self::SSL => K8EncryptionEnum::SSL,
            }
        }
    }

    impl From<K8SpuType> for SpuType {
        fn from(kv_spu_type: K8SpuType) -> Self {
            match kv_spu_type {
                K8SpuType::Managed => SpuType::Managed,
                K8SpuType::Custom => SpuType::Custom,
            }
        }
    }

    impl Into<K8SpuType> for SpuType {
        fn into(self) -> K8SpuType {
            match self {
                SpuType::Managed => K8SpuType::Managed,
                SpuType::Custom => K8SpuType::Custom,
            }
        }
    }
}
