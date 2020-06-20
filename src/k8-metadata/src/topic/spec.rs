//!
//! # Topic Spec
//!
//! Interface to the Topic metadata spec in K8 key value store
//!
use k8_obj_metadata::Crd;
use k8_obj_metadata::Spec;
use k8_obj_metadata::DefaultHeader;

use serde::Deserialize;
use serde::Serialize;

use super::TOPIC_API;

use super::TopicStatus;

// -----------------------------------
// Data Structures
// -----------------------------------

impl Spec for TopicSpec {
    type Status = TopicStatus;
    type Header = DefaultHeader;

    fn metadata() -> &'static Crd {
        &TOPIC_API
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopicSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_rack_assignment: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_replica_assignment: Option<Vec<Partition>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Partition {
    pub partition: PartitionDetails,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartitionDetails {
    pub id: i32,
    pub replicas: Vec<i32>, //spu_ids
}

// -----------------------------------
// Implementation
// -----------------------------------

impl TopicSpec {
    pub fn new(
        partitions: Option<i32>,
        replication_factor: Option<i32>,
        ignore_rack_assignment: Option<bool>,
        custom_replica_assignment: Option<Vec<Partition>>,
    ) -> Self {
        TopicSpec {
            partitions,
            replication_factor,
            ignore_rack_assignment,
            custom_replica_assignment,
        }
    }
}

impl Partition {
    pub fn new(id: i32, replicas: Vec<i32>) -> Self {
        Partition {
            partition: PartitionDetails { id, replicas },
        }
    }

    pub fn id(&self) -> i32 {
        self.partition.id
    }

    pub fn replicas(&self) -> &Vec<i32> {
        &self.partition.replicas
    }

    pub fn replica_cnt(&self) -> i32 {
        self.partition.replicas.len() as i32
    }
}


#[cfg(feature ="kf")]
mod convert {

    use k8_metadata::topic::TopicSpec as K8TopicSpec;
use k8_metadata::topic::Partition as K8Partition;

impl From<TopicSpec> for K8TopicSpec {
    fn from(spec: TopicSpec) -> Self {
        match spec {
            TopicSpec::Computed(computed_param) => K8TopicSpec::new(
                Some(computed_param.partitions),
                Some(computed_param.replication_factor),
                Some(computed_param.ignore_rack_assignment),
                None,
            ),
            TopicSpec::Assigned(assign_param) => K8TopicSpec::new(
                None,
                None,
                None,
                Some(replica_map_to_k8_partition(assign_param)),
            ),
        }
    }
}


/// Translate Fluvio Replica Map to K8 Partitions to KV store notification
fn replica_map_to_k8_partition(partition_maps: PartitionMaps) -> Vec<K8Partition> {
    let mut k8_partitions: Vec<K8Partition> = vec![];
    for partition in partition_maps.maps() {
        k8_partitions.push(K8Partition::new(partition.id, partition.replicas.clone()));
    }
    k8_partitions
}

}