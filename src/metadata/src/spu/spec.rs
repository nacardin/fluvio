//!
//! # Spu Spec
//!
//! Spu Spec metadata information cached locally.
//!
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::fmt;

use flv_util::socket_helpers::EndPoint as SocketEndPoint;
use flv_util::socket_helpers::EndPointEncryption;
use flv_types::defaults::{SPU_PRIVATE_HOSTNAME, SPU_PRIVATE_PORT};
use flv_types::defaults::SPU_PUBLIC_PORT;
use flv_types::SpuId;
use flv_util::socket_helpers::ServerAddress;

use kf_protocol::derive::{Decode, Encode};


// -----------------------------------
// Data Structures
// -----------------------------------

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub struct SpuSpec {
    pub id: SpuId,
    pub spu_type: SpuType,
    pub public_endpoint: IngressPort,
    pub private_endpoint: Endpoint,
    pub rack: Option<String>,
}

impl Default for SpuSpec {
    fn default() -> Self {
        SpuSpec {
            id: -1,
            spu_type: SpuType::default(),
            public_endpoint: IngressPort {
                port: SPU_PUBLIC_PORT,
                ..Default::default()
            },
            private_endpoint: Endpoint {
                port: SPU_PRIVATE_PORT,
                host: SPU_PRIVATE_HOSTNAME.to_string(),
                encryption: EncryptionEnum::default(),
            },
            rack: None,
        }
    }
}

impl From<SpuId> for SpuSpec {
    fn from(spec: SpuId) -> Self {
        Self::new(spec)
    }
}


impl SpuSpec {
    /// Given an Spu id generate a new SpuSpec
    pub fn new(id: SpuId) -> Self {
        let mut spec = Self::default();
        spec.id = id;
        spec
    }

    pub fn set_custom(mut self) -> Self {
        self.spu_type = SpuType::Custom;
        self
    }

    /// Return type label in String format
    pub fn type_label(&self) -> String {
        match self.spu_type {
            SpuType::Managed => "managed".to_owned(),
            SpuType::Custom => "custom".to_owned(),
        }
    }

    /// Return custom type: true for custom, false otherwise
    pub fn is_custom(&self) -> bool {
        match self.spu_type {
            SpuType::Managed => false,
            SpuType::Custom => true,
        }
    }

    pub fn private_server_address(&self) -> ServerAddress {
        let private_ep = &self.private_endpoint;
        ServerAddress {
            host: private_ep.host.clone(),
            port: private_ep.port,
        }
    }
}

#[derive(Decode, Encode, Default, Debug, Clone, PartialEq)]
pub struct IngressPort {
    pub port: u16,
    pub ingress: Vec<IngressAddr>,
    pub encryption: EncryptionEnum,
}

impl fmt::Display for IngressPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host_string(), self.port)
    }
}


impl IngressPort {
    pub fn from_port_host(port: u16, host: String) -> Self {
        Self {
            port: port,
            ingress: vec![IngressAddr {
                hostname: Some(host),
                ip: None,
            }],
            encryption: EncryptionEnum::PLAINTEXT,
        }
    }

    // return any host whether it is IP or String
    pub fn host(&self) -> Option<String> {
        if self.ingress.len() == 0 {
            None
        } else {
            self.ingress[0].host()
        }
    }

    pub fn host_string(&self) -> String {
        match self.host() {
            Some(host_val) => host_val,
            None => "".to_owned(),
        }
    }
}

#[derive(Decode, Encode, Default, Debug, Clone, PartialEq)]
pub struct IngressAddr {
    pub hostname: Option<String>,
    pub ip: Option<String>,
}

impl IngressAddr {
    pub fn host(&self) -> Option<String> {
        if let Some(name) = &self.hostname {
            Some(name.clone())
        } else {
            if let Some(ip) = &self.ip {
                Some(ip.clone())
            } else {
                None
            }
        }
    }
}

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub struct Endpoint {
    pub port: u16,
    pub host: String,
    pub encryption: EncryptionEnum,
}


impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl TryFrom<&Endpoint> for SocketEndPoint {
    type Error = IoError;

    fn try_from(endpoint: &Endpoint) -> Result<Self, Self::Error> {
        flv_util::socket_helpers::host_port_to_socket_addr(&endpoint.host, endpoint.port).map(
            |addr| SocketEndPoint {
                addr,
                encryption: EndPointEncryption::PLAINTEXT,
            },
        )
    }
}

#[allow(dead_code)]
impl TryFrom<&Endpoint> for std::net::SocketAddr {
    type Error = IoError;

    fn try_from(endpoint: &Endpoint) -> Result<Self, Self::Error> {
        flv_util::socket_helpers::host_port_to_socket_addr(&endpoint.host, endpoint.port)
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 0,
            encryption: EncryptionEnum::default(),
        }
    }
}

impl Endpoint {
    pub fn from_port_host(port: u16, host: String) -> Self {
        Self {
            port: port,
            host: host,
            encryption: EncryptionEnum::PLAINTEXT,
        }
    }

    
}

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub enum EncryptionEnum {
    PLAINTEXT,
    SSL,
}

impl Default for EncryptionEnum {
    fn default() -> Self {
        EncryptionEnum::PLAINTEXT
    }
}

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub enum SpuType {
    Managed,
    Custom,
}

impl Default for SpuType {
    fn default() -> Self {
        SpuType::Managed
    }
}
