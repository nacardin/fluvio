//!
//! Spu Group
//!

use flv_metadata::spg::SpuGroupSpec;

use super::*;

pub type SpuGroupKV = MetadataStoreObject<SpuGroupSpec>;

pub type SpuGroupLocalStore = LocalStore<SpuGroupSpec>;
