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

    use flv_metadata::spu::SpuSpec as FlvSpuSpec;
    use flv_metadata::spu::SpuType as FlvSpuType;
    use flv_metadata::spu::EncryptionEnum as FlvEncryptionEnum;
    use flv_metadata::spu::Endpoint as FlvEndpoint;
    use flv_metadata::spu::IngressPort as FlvIngressPort;
    use flv_metadata::spu::IngressAddr as FlvIngressAddr;
    use super::*;

    impl Into<FlvSpuSpec> for SpuSpec {
        fn into(self) -> FlvSpuSpec {
            // convert spu-type, defaults to Custom for none
            let spu_type = if let Some(kv_spu_type) = self.spu_type {
                kv_spu_type.into()
            } else {
                FlvSpuType::Custom
            };

            // spu spec
            FlvSpuSpec {
                id: self.spu_id,
                spu_type,
                public_endpoint: self.public_endpoint.into(),
                private_endpoint: self.private_endpoint.into(),
                rack: self.rack
            }
        }
    }

    impl From<FlvSpuSpec> for SpuSpec {
        fn from(spec: FlvSpuSpec) -> Self {
            Self {
                spu_id: spec.id,
                spu_type: Some(spec.spu_type.into()),
                public_endpoint: spec.public_endpoint.into(),
                private_endpoint: spec.private_endpoint.into(),
                rack: spec.rack,
            }
        }
    }

    impl Into<FlvIngressPort> for IngressPort {
        fn into(self) -> FlvIngressPort {
            FlvIngressPort {
                port: self.port,
                ingress: self.ingress.into_iter().map(|a| a.into()).collect(),
                encryption: self.encryption.into(),
            }
        }
    }

    impl From<FlvIngressPort> for IngressPort {
        fn from(port: FlvIngressPort) -> Self {
            Self {
                port: port.port,
                ingress: port.ingress.into_iter().map(|a| a.into()).collect(),
                encryption: port.encryption.into(),
            }
        }
    }

    impl Into<FlvIngressAddr> for IngressAddr {
        fn into(self) -> FlvIngressAddr {
            FlvIngressAddr {
                hostname: self.hostname,
                ip: self.ip,
            }
        }
    }

    impl From<FlvIngressAddr> for IngressAddr {
        fn from(addr: FlvIngressAddr) -> Self {
            Self {
                hostname: addr.hostname,
                ip: addr.ip,
            }
        }
    }

    impl Into<FlvEndpoint> for Endpoint {
        fn into(self) -> FlvEndpoint {
            FlvEndpoint {
                port: self.port,
                host: self.host,
                encryption: self.encryption.into(),
            }
        }
    }

    impl From<FlvEndpoint> for Endpoint {
        fn from(pt: FlvEndpoint) -> Endpoint {
            Self {
                port: pt.port,
                host: pt.host,
                encryption: pt.encryption.into(),
            }
        }
    }


    impl Into<FlvEncryptionEnum> for EncryptionEnum {
        fn into(self) -> FlvEncryptionEnum {
            match self {
                Self::PLAINTEXT => FlvEncryptionEnum::PLAINTEXT,
                Self::SSL => FlvEncryptionEnum::SSL,
            }
        }
    }

    impl From<FlvEncryptionEnum> for EncryptionEnum {
        fn from(enc: FlvEncryptionEnum) -> Self {
            match enc {
                FlvEncryptionEnum::PLAINTEXT => Self::PLAINTEXT,
                FlvEncryptionEnum::SSL => Self::SSL,
            }
        }
    }

    impl Into<FlvSpuType> for SpuType {
        fn into(self) -> FlvSpuType {
            match self {
                Self::Managed => FlvSpuType::Managed,
                Self::Custom => FlvSpuType::Custom,
            }
        }
    }

    impl From<FlvSpuType> for SpuType {
        fn from(ty: FlvSpuType) -> SpuType {
            match ty {
                FlvSpuType::Managed => Self::Managed,
                FlvSpuType::Custom => Self::Custom,
            }
        }
    }
}
