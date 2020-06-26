mod error;
mod spu_old;
mod replica;
//mod sc;
pub mod client;
pub mod admin;
pub mod consumer;
pub mod producer;
pub mod config;

pub mod query_params;

pub use error::ClientError;
pub use spu_old::SpuReplicaLeader;
pub use replica::*;

pub const MAX_FETCH_BYTES: u32 = 1000000;

/// re-export metadata from sc-api
pub mod metadata {

    pub mod topic {
        pub use sc_api::topic::*;
    }

    pub mod spu {
        pub use sc_api::spu::*;
    }

    pub mod spg {
        pub use sc_api::spg::*;
    }

    pub mod objects {
        pub use sc_api::objects::*;
    }
}

pub mod kf {
    pub mod api {
        pub use kf_protocol::api::*;
    }

    pub mod message {
        pub use kf_protocol::message::*;
    }
}