
use kf_protocol::derive::{Decode, Encode};


#[derive(Encode, Decode, Default, Debug, Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "camelCase"))]
pub struct SpuGroupSpec {

    /// The number of replicas for the spu group
    pub replicas: u16,

    /// The base spu id that the spu group uses to increment the spu ids
    /// Note: Spu id is a globally unique resource and it cannot be shared
    pub min_id: i32,

    /// Configuration elements to be applied to each SPUs in the group
    pub spu_config: SpuConfig,

}


#[derive(Encode, Decode, Default, Debug, Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "camelCase"))]
pub struct SpuConfig {
    pub rack: Option<String>,
    pub replication: Option<ReplicationConfig>,
    pub storage: Option<StorageConfig>,
    pub env: Vec<EnvVar>,
}

#[derive(Encode, Decode, Default, Debug, Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "camelCase"))]
pub struct ReplicationConfig {
    pub in_sync_replica_min: Option<u16>,
}

#[derive(Encode, Decode, Debug, Default, Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "camelCase"))]
pub struct StorageConfig {
    pub log_dir: Option<String>,
    pub size: Option<String>,
}

impl StorageConfig {
    pub fn log_dir(&self) -> String {
        self.log_dir.clone().unwrap_or("/tmp/fluvio".to_owned())
    }

    pub fn size(&self) -> String {
        self.size.clone().unwrap_or("1Gi".to_owned())
    }
}



#[derive(Encode, Decode, Default, Debug, Clone )]
#[cfg_attr(feature = "use_serde", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "camelCase"))]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}