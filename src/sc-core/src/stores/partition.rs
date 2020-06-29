//!
//! # Partition & Partitions Metadata
//!
//! Partition metadata information on cached in the local Controller.
//!

use std::sync::Arc;

use log::debug;

use internal_api::messages::Replica;
use flv_metadata::partition::ReplicaKey;
use flv_metadata::partition::{PartitionSpec, PartitionStatus};
use flv_types::SpuId;
use sc_api::metadata::*;

use super::*;


pub type SharedPartitionStore = Arc<PartitionLocalStore>;



// -----------------------------------
// Data Structures
// -----------------------------------
pub type PartitionKV = KVObject<PartitionSpec>;

// -----------------------------------
// Partition - Implementation
// -----------------------------------

impl PartitionKV {
    /// create new partiton with replica map.
    /// first element of replicas is leader
    pub fn with_replicas(key: ReplicaKey, replicas: Vec<SpuId>) -> Self {
        let spec: PartitionSpec = replicas.into();
        Self::new(key, spec, PartitionStatus::default())
    }
}

impl<S> From<((S, i32), Vec<i32>)> for PartitionKV
where
    S: Into<String>,
{
    fn from(partition: ((S, i32), Vec<i32>)) -> Self {
        let (replica_key, replicas) = partition;
        Self::with_replicas(replica_key.into(), replicas)
    }
}

pub type PartitionLocalStore = LocalStore<PartitionSpec>;

// -----------------------------------
// Partitions - Implementation
// -----------------------------------

impl PartitionLocalStore {
    pub async fn names(&self) -> Vec<ReplicaKey> {
        self.read().await.keys().cloned().collect()
    }

    pub async fn topic_partitions(&self, topic: &str) -> Vec<PartitionKV> {
        let mut res: Vec<PartitionKV> = Vec::default();
        for (name, partition) in self.read().await.iter() {
            if name.topic == topic {
                res.push(partition.clone());
            }
        }
        res
    }

    /// find all partitions that has spu in the replicas
    pub async fn partition_spec_for_spu(&self, target_spu: i32) -> Vec<(ReplicaKey, PartitionSpec)> {
        let mut res = vec![];
        for (name, partition) in self.read().await.iter() {
            if partition.spec.replicas.contains(&target_spu) {
                res.push((name.clone(), partition.spec.clone()));
            }
        }
        res
    }

    pub async fn count_topic_partitions(&self, topic: &str) -> i32 {
        let mut count: i32 = 0;
        for (name, _) in self.read().await.iter() {
            if name.topic == topic {
                count += 1;
            }
        }
        count
    }

    // return partitions that belong to this topic
    #[allow(dead_code)]
    async fn topic_partitions_list(&self, topic: &str) -> Vec<ReplicaKey> {
        self.read()
            .await
            .keys()
            .filter_map(|name| {
                if &name.topic == topic {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// replica msg for target spu
    pub async fn replica_for_spu(&self, target_spu: SpuId) -> Vec<Replica> {
        let msgs: Vec<Replica> = self
            .partition_spec_for_spu(target_spu)
            .await
            .into_iter()
            .map(|(replica_key, partition_spec)| {
                Replica::new(replica_key, partition_spec.leader, partition_spec.replicas)
            })
            .collect();
        debug!(
            "{} computing replic msg for spuy: {}, msg: {}",
            self,
            target_spu,
            msgs.len()
        );
        msgs
    }


    pub async fn leaders(&self) -> Vec<ReplicaLeader> {
    
        self.read()
            .await
            .iter()
            .map(|(key, value)| ReplicaLeader { id: key.clone(), leader: value.spec.leader })
            .collect()
    }


    pub async fn table_fmt(&self) -> String {
        let mut table = String::new();

        let partition_hdr = format!(
            "{n:<18}   {l:<6}  {r}\n",
            n = "PARTITION",
            l = "LEADER",
            r = "LIVE-REPLICAS",
        );
        table.push_str(&partition_hdr);

        for (name, partition) in self.read().await.iter() {
            let mut leader = String::from("-");
            let mut _lrs = String::from("[]");

            if partition.spec.leader >= 0 {
                leader = format!("{}", partition.spec.leader);
                //   lrs = partition.live_replicas_str();
            }
            let row = format!("{n:<18} {l:<6} \n", n = name.to_string(), l = leader,);
            table.push_str(&row);
        }

        table
    }

    pub fn bulk_add<S>(&self, partitions: Vec<((S, i32), Vec<i32>)>)
    where
        S: Into<String>,
    {
        for (replica_key, replicas) in partitions.into_iter() {
            let partition: PartitionKV = (replica_key, replicas).into();
            self.insert(partition);
        }
    }
}

impl<S> From<Vec<((S, i32), Vec<i32>)>> for PartitionLocalStore
where
    S: Into<String>,
{
    fn from(partitions: Vec<((S, i32), Vec<i32>)>) -> Self {
        let store = Self::default();
        store.bulk_add(partitions);
        store
    }
}

#[cfg(test)]
pub mod test {

    use flv_future_aio::test_async;
    use super::PartitionLocalStore;

    #[test_async]
    async fn test_partitions_to_replica_msgs() -> Result<(),()> {
        let partitions = PartitionLocalStore::default();
        partitions.bulk_add(vec![(("topic1", 0), vec![10, 11, 12])]);

        let replica_msg = partitions.replica_for_spu(10).await;
        assert_eq!(replica_msg.len(), 1);
        Ok(())
    }
}
