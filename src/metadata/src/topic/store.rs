
use flv_types::ReplicaMap;

use crate::store::*;
use crate::core::*;
use super::*;

pub type TopicMetadata<C> = MetadataStoreObject<TopicSpec,C>;
pub type TopicLocalStore<C> = LocalStore<TopicSpec,C>;
pub type DefaultTopicMd = TopicMetadata<String>;
pub type DefaultTopicLocalStore = TopicLocalStore<String>;



impl <C>std::fmt::Display for TopicMetadata<C> 
    where C: MetadataItem
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.spec {
            TopicSpec::Assigned(partition_map) => write!(f, "assigned::{}", partition_map),
            TopicSpec::Computed(param) => write!(f, "computed::({})", param),
        }
    }
}




impl<C> TopicMetadata<C> 
    where C: MetadataItem
{
    pub fn is_provisioned(&self) -> bool {
        self.status.is_resolution_provisioned()
    }

    pub fn replica_map(&self) -> &ReplicaMap {
        &self.status.replica_map
    }

    pub fn reason(&self) -> &String {
        &self.status.reason
    }
}



// -----------------------------------
// Topics - Implementation
// -----------------------------------

impl <C>TopicLocalStore<C> 
    where C: MetadataItem
{
    pub async fn topic(&self, topic_name: &str) -> Option<TopicMetadata<C>> {
        match self.read().await.get(topic_name) {
            Some(topic) => Some(topic.clone()),
            None => None,
        }
    }

    pub async fn table_fmt(&self) -> String {
        let mut table = String::new();

        let topic_hdr = format!(
            "{n:<18}   {t:<8}  {p:<5}  {s:<5}  {g:<8}  {l:<14}  {m:<10}  {r}\n",
            n = "TOPIC",
            t = "TYPE",
            p = "PART",
            s = "FACT",
            g = "IGN-RACK",
            l = "RESOLUTION",
            m = "R-MAP-ROWS",
            r = "REASON",
        );
        table.push_str(&topic_hdr);

        for (name, topic) in self.read().await.iter() {
            let topic_row = format!(
                "{n:<18}  {t:^8}  {p:^5}  {s:^5}  {g:<8}  {l:^14}  {m:^10}  {r}\n",
                n = name.clone(),
                t = topic.spec.type_label(),
                p = topic.spec.partitions_display(),
                s = topic.spec.replication_factor_display(),
                g = topic.spec.ignore_rack_assign_display(),
                l = topic.status.resolution().resolution_label(),
                m = topic.status.replica_map_cnt_str(),
                r = topic.reason(),
            );
            table.push_str(&topic_row);
        }

        table
    }
}