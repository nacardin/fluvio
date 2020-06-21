//!
//! # Spu Spec
//!
//! Spu Spec metadata information cached locally.
//!
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::fmt;

use flv_util::socket_helpers::EndPoint as SocketEndPoint;
use flv_util::socket_helpers::EndPointEncryption;
use flv_types::defaults::{SPU_PRIVATE_HOSTNAME, SPU_PRIVATE_PORT};
use flv_types::defaults::SPU_PUBLIC_PORT;
use flv_types::SpuId;
use flv_util::socket_helpers::ServerAddress;

use kf_protocol::derive::{Decode, Encode};
use kf_protocol::{Decoder, Encoder};
use kf_protocol::bytes::{Buf, BufMut};
use kf_protocol::Version;

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


// -----------------------------------
// Data Structures - CustomSpu
// -----------------------------------

#[derive(Debug)]
pub enum CustomSpu {
    Name(String),
    Id(i32),
}

// -----------------------------------
// Implementation - CustomSpu
// -----------------------------------
impl Default for CustomSpu {
    fn default() -> CustomSpu {
        CustomSpu::Name("".to_string())
    }
}

impl Encoder for CustomSpu {
    // compute size
    fn write_size(&self, version: Version) -> usize {
        let type_size = (0 as u8).write_size(version);
        match self {
            CustomSpu::Name(name) => type_size + name.write_size(version),
            CustomSpu::Id(id) => type_size + id.write_size(version),
        }
    }

    // encode match
    fn encode<T>(&self, dest: &mut T, version: Version) -> Result<(), IoError>
    where
        T: BufMut,
    {
        // ensure buffer is large enough
        if dest.remaining_mut() < self.write_size(version) {
            return Err(IoError::new(
                ErrorKind::UnexpectedEof,
                format!(
                    "not enough capacity for custom spu len of {}",
                    self.write_size(version)
                ),
            ));
        }

        match self {
            CustomSpu::Name(name) => {
                let typ: u8 = 0;
                typ.encode(dest, version)?;
                name.encode(dest, version)?;
            }
            CustomSpu::Id(id) => {
                let typ: u8 = 1;
                typ.encode(dest, version)?;
                id.encode(dest, version)?;
            }
        }

        Ok(())
    }
}

impl Decoder for CustomSpu {
    fn decode<T>(&mut self, src: &mut T, version: Version) -> Result<(), IoError>
    where
        T: Buf,
    {
        let mut value: u8 = 0;
        value.decode(src, version)?;
        match value {
            0 => {
                let mut name: String = String::default();
                name.decode(src, version)?;
                *self = CustomSpu::Name(name)
            }
            1 => {
                let mut id: i32 = 0;
                id.decode(src, version)?;
                *self = CustomSpu::Id(id)
            }
            _ => {
                return Err(IoError::new(
                    ErrorKind::UnexpectedEof,
                    format!("invalid value for Custom Spu: {}", value),
                ))
            }
        }

        Ok(())
    }
}