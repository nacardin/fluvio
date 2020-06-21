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


    // update metadata async
    UpdateMetadata = 1000,
    //  FlvUpdateSpuMetadata = 1001,
    //  FlvUpdateReplicaMetadata = 1002,

    // Topics
    CreateTopic = 2001,
    DeleteTopic = 2002,
    FetchTopics = 2003,
    TopicComposition = 2004,

    // Custom SPUs
    RegisterCustomSpu = 2005,
    UnregisterCustomSpu = 2006,
    FetchSpus = 2007,

    // SPU Groups
    CreateSpuGroup = 2008,
    DeleteSpuGroup = 2009,
    FetchSpuGroups = 2010,
}

impl Default for ScPublicApiKey {
    fn default() -> Self {
        Self::ApiVersion
    }
}
