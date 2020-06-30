//!
//! Spu Group
//!

use flv_metadata::spg::SpuGroupSpec;
use flv_metadata::k8::metadata::ObjectMeta;

use super::*;

pub type SpuGroupMetadata<C> = MetadataStoreObject<SpuGroupSpec,C>;

pub type SpuGroupLocalStore<C> = LocalStore<SpuGroupSpec,C>;

pub type K8SpuGroupLocalStore = SpuGroupLocalStore<ObjectMeta>;
pub type K8SpuGroupMd = SpuGroupMetadata<ObjectMeta>;