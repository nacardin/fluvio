//!
//! # Fluvio -- Topic/Partition Parameters
//!
//! Intermediate structure to collect metadata information
//!
use kf_protocol::api::Offset;

/// Fetch Logs parameters
#[derive(Debug)]
pub struct FetchLogsParam {
    pub topic: String,
    pub max_bytes: i32,

    pub partitions: Vec<PartitionParam>,
}

/// Topic/Partition parameters
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPartitionParam {
    pub topic_name: String,

    pub leaders: Vec<LeaderParam>,
}

/// Replica Leader parameters
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderParam {
    pub leader_id: i32,
    pub server_addr: String,

    pub partitions: Vec<PartitionParam>,
}

/// Partition parameters
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartitionParam {
    pub partition_idx: i32,
    pub offset: Offset,
    pub epoch: i32,
}

pub use topic_partition::*;

mod topic_partition {

    use serde::Deserialize;
    use std::io::Error as IoError;
    use std::io::ErrorKind;
    use std::fs::read_to_string;
    use std::path::Path;

    use sc_api::topic::PartitionMap;

    #[derive(Debug)]
    pub enum ReplicaConfig {
        // replica assignment
        Assigned(Partitions),

        // partitions, replication, ignore_rack_assignment
        Computed(i32, i16, bool),
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Partitions {
        partitions: Vec<Partition>,
    }

    impl Partitions {
        #[allow(unused)]
        pub fn new(partitions: Vec<Partition>) -> Self {
            Self { partitions }
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Partition {
        id: i32,
        replicas: Vec<i32>,
    }

    impl Partition {
        pub fn new(id: i32, replicas: Vec<i32>) -> Self {
            Self { id, replicas }
        }
    }

    impl Partitions {
        /// Read and decode the json file into Replica Assignment map
        pub fn file_decode<T: AsRef<Path>>(path: T) -> Result<Self, IoError> {
            let file_str: String = read_to_string(path)?;
            serde_json::from_str(&file_str)
                .map_err(|err| IoError::new(ErrorKind::InvalidData, format!("{}", err)))
        }

        
        // Encode Replica Assignment map into Fluvio format
        pub fn sc_encode(&self) -> Vec<PartitionMap> {
            let mut partition_map: Vec<PartitionMap> = vec![];
            for partition in &self.partitions {
                partition_map.push(PartitionMap {
                    id: partition.id,
                    replicas: partition.replicas.clone(),
                })
            }
            partition_map
        }
    }
}
