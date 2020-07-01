
use std::sync::Arc;

use log::debug;

use internal_api::messages::Replica;
use flv_metadata::partition::ReplicaKey;
use flv_metadata::k8::metadata::ObjectMeta;
use flv_metadata::partition::{PartitionSpec, PartitionStatus};
use flv_types::SpuId;
use sc_api::metadata::*;

use super::*;

pub type SharedPartitionStore<C> = Arc<PartitionLocalStore<C>>;


pub type PartitionMetadata<C> = MetadataStoreObject<PartitionSpec,C>;
pub type K8PartitionMd = PartitionMetadata<ObjectMeta>;
pub type DefaultPartitionMd = PartitionMetadata<String>;
pub type PartitionLocalStore<C> = LocalStore<PartitionSpec,C>;
pub type K8PartitionLocalStore = PartitionLocalStore<ObjectMeta>;


