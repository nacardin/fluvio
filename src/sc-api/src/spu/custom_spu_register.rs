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

// -----------------------------------
// FlvRegisterCustomSpusRequest
// -----------------------------------

#[derive(Encode, Decode, Default, Debug)]
pub struct FlvRegisterCustomSpuRequest {

    pub name: String,
    pub spec: SpuSpec,
    pub dry_run: bool
}

impl FlvRegisterCustomSpuRequest {

    pub fn new(id: i32,
        name: String,
        public_server: ServerAddress,
        private_server: ServerAddress,
        rack: Option<String>) -> Self {

        Self {
            name,
            spec: SpuSpec {
                id,
                public_server: IngressPort {
                    host: public_server.host,
                    port: public_server.port,
                    ..Defaults::default()
                },
                private_server: Endpoint {
                    host: private_server.host.clone(),
                    port: private_server.port,
                },
                rack,
            }
        }
    }
}

impl Request for FlvRegisterCustomSpuRequest {
    const API_KEY: u16 = ScPublicApiKey::FlvRegisterCustomSpu as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = FlvStatus;
}
