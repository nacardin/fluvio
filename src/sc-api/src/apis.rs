//!
//! # SC Api Keys
//!
//! Stores Api Keys supported by the SC.
//!

use kf_protocol::derive::Encode;
use kf_protocol::derive::Decode;

/// API call from client to SPU
#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
#[repr(u16)]
pub enum ScPublicApiKey {
    // Mixed
    ApiVersion = 18,


    Create = 1001,
   // Delete = 1002,
   // Fetch = 1003,

    TopicComposition = 2000,
    UpdateMetadata = 2001,
}

impl Default for ScPublicApiKey {
    fn default() -> Self {
        Self::ApiVersion
    }
}
