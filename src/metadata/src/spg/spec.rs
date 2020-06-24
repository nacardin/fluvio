
use kf_protocol::derive::{Decode, Encode};


#[derive(Encode, Decode, Default, Debug, Clone)]
pub struct SpuGroupSpec {


    /// The number of replicas for the spu group
    pub replicas: u16,

    /// The base spu id that the spu group uses to increment the spu ids
    /// Note: Spu id is a globally unique resource and it cannot be shared
    pub min_id: i32,

    /// Configuration elements to be applied to each SPUs in the group
    pub config: GroupConfig,

    /// The rack to be used for all SPUs in the group. Racks are used by
    /// replication assignment algorithm
    pub rack: Option<String>,
}




/// equivalent to spu template
#[derive(Encode, Decode, Default, Debug, Clone )]
pub struct GroupConfig {
    pub storage: Option<StorageConfig>,
    pub replication: Option<ReplicationConfig>,
    pub env: Vec<EnvVar>,
}

#[derive(Encode, Decode, Default, Debug, Clone )]
pub struct StorageConfig {
    pub log_dir: Option<String>,
    pub size: Option<String>,
}

#[derive(Encode, Decode, Default, Debug, Clone )]
pub struct ReplicationConfig {
    pub in_sync_replica_min: Option<u16>,
}

#[derive(Encode, Decode, Default, Debug, Clone )]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}