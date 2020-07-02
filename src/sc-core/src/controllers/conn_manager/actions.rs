//!
//! # Connection actions
//!
//! Actions are received through check dispatcher channel and are forwarded to
//! Connection manager for processing.
//!
use flv_types::SpuId;
use flv_metadata::spu::SpuSpec;
use flv_metadata::partition::PartitionSpec;
use flv_metadata::partition::ReplicaKey;

/// Change in connection status
#[derive(Debug, PartialEq, Clone)]
pub enum SpuConnectionStatusChange {
    Off(SpuId),
    On(SpuId),
}

impl SpuConnectionStatusChange {
    pub fn spu_id(&self) -> SpuId {
        match self {
            SpuConnectionStatusChange::Off(id) => *id,
            SpuConnectionStatusChange::On(id) => *id,
        }
    }
}

impl std::fmt::Display for SpuConnectionStatusChange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpuConnectionStatusChange::Off(id) => write!(f, "SPU {} Off", id),
            SpuConnectionStatusChange::On(id) => write!(f, "SPU {} On", id),
        }
    }
}

#[derive(Debug, PartialEq,Clone)]
pub enum SpuSpecChange {
    Add(SpuSpec),          // New Spec
    Mod(SpuSpec, SpuSpec), // Update SPU spec (new,old)
    Remove(SpuSpec),
}

#[derive(Debug, PartialEq,Clone)]
pub enum PartitionSpecChange {
    Add(ReplicaKey, PartitionSpec),
    Mod(ReplicaKey, PartitionSpec, PartitionSpec),
    #[allow(unused)]
    Remove(ReplicaKey, PartitionSpec),
}

/// Request to made to Connection Manager
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionRequest {
    Spu(SpuSpecChange),
    Partition(PartitionSpecChange),
    RefreshSpu(SpuId), // Refresh SPU with it' metadata including SPU and Replica
}

/// notify client of changes
#[derive(Debug,Clone)]
pub enum ClientNotification {
    SPU(sc_api::metadata::UpdateSpuResponse),
    Replica(sc_api::metadata::UpdateReplicaResponse)
}