
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