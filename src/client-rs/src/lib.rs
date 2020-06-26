mod error;
mod client;
mod admin;
mod consumer;
mod producer;
pub mod config;

pub mod params;

pub use error::ClientError;
pub use config::ClusterConfig;




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