//!
//! # Create Custom SPU
//!
//! Public API to request the SC to create one or more custom spus
//!
//!
use kf_protocol::api::Request;
use kf_protocol::derive::{Decode, Encode};
use flv_util::socket_helpers::ServerAddress;
use flv_metadata::spu::*;

use crate::FlvStatus;
use crate::ScPublicApiKey;
use crate::AdminRequest;

#[derive(Encode, Decode, Default, Debug)]
pub struct RegisterCustomSpuRequest {

    pub name: String,
    pub spec: SpuSpec,
    pub dry_run: bool
}

impl Request for RegisterCustomSpuRequest {
    const API_KEY: u16 = ScPublicApiKey::RegisterCustomSpu as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}

impl AdminRequest for RegisterCustomSpuRequest{}

impl RegisterCustomSpuRequest {

    pub fn new(id: i32,
        name: String,
        public_server: ServerAddress,
        private_server: ServerAddress,
        rack: Option<String>) -> Self {

        Self {
            name,
            spec: SpuSpec {
                id,
                spu_type: SpuType::Custom,
                public_endpoint: IngressPort::from_port_host(
                    public_server.port,
                    public_server.host,
                ),
                private_endpoint: Endpoint::from_port_host(
                    private_server.port,
                    private_server.host
                ),
                rack,
            },
            ..Default::default()
        }
    }
}

