use log::debug;

use flv_types::ReplicaMap;

use crate::store::*;
use crate::core::*;
use crate::partition::store::*;
use crate::partition::*;
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

    /// create new partitions from my replica map if it doesn't exists
    /// from partition store
    pub async fn create_new_partitions(
        &self,
        partition_store: &PartitionLocalStore<C>,
    ) -> Vec<PartitionMetadata<C>> {

        let mut partitions = vec![];
        for (idx, replicas) in self.status.replica_map.iter() {
            let replica_key = ReplicaKey::new(self.key(), *idx);
            debug!("Topic: {} creating partition: {}", self.key(), replica_key);
            if !partition_store.contains_key(&replica_key).await {
                partitions.push(
                    PartitionMetadata::with_spec(replica_key, replicas.clone().into())
                        .with_context(self.ctx.create_child()),
                )
            }
        }
        partitions
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


mod test {
    
    use flv_future_aio::test_async;

    use super::*;

    #[test]
    fn test_topic_replica_map() {
        // empty replica map
        let topic1 = DefaultTopicMd::new("Topic-1", (1, 1, false).into(), TopicStatus::default());
        assert_eq!(topic1.replica_map().len(), 0);

        // replica map with 2 partitions
        let topic2 = DefaultTopicMd::new(
            "Topic-2",
            (1, 1, false).into(),
            TopicStatus::new(
                TopicResolution::Provisioned,
                vec![vec![0, 1], vec![1, 2]],
                "".to_owned(),
            ),
        );
        assert_eq!(topic2.replica_map().len(), 2);
    }

    #[test]
    fn test_update_topic_status_objects() {
        // create topic 1
        let mut topic1 = DefaultTopicMd::new("Topic-1", (2, 2, false).into(), TopicStatus::default());
        assert_eq!(topic1.status.resolution, TopicResolution::Init);

        // create topic 2
        let topic2 = DefaultTopicMd::new(
            "Topic-1",
            (2, 2, false).into(),
            TopicStatus::new(
                TopicResolution::Provisioned,
                vec![vec![0, 1], vec![1, 2]],
                "".to_owned(),
            ),
        );

        // test update individual components
        topic1.status.set_replica_map(topic2.replica_map().clone());
        topic1.status.reason = topic2.reason().clone();
        topic1.status.resolution = (&topic2.status.resolution).clone();

        // topics should be identical
        assert_eq!(topic1, topic2);
    }

    #[test_async]
    async fn topic_list_insert() -> Result<(), ()> {
        // create topics
        let topic1 = DefaultTopicMd::new("Topic-1", (1, 1, false).into(), TopicStatus::default());
        let topic2 = DefaultTopicMd::new("Topic-2", (2, 2, false).into(), TopicStatus::default());

        let topics = DefaultTopicLocalStore::default();
        topics.insert(topic1).await;
        topics.insert(topic2).await;

        assert_eq!(topics.count().await, 2);
        Ok(())
    }



    #[test_async]
    async fn test_topics_in_pending_state() -> Result<(), ()> {
        let topics = DefaultTopicLocalStore::default();

        // resolution: Init
        let topic1 = DefaultTopicMd::new("Topic-1", (1, 1, false).into(), TopicStatus::default());
        assert_eq!(topic1.status.is_resolution_initializing(), true);

        // resolution: Pending
        let topic2 = DefaultTopicMd::new(
            "Topic-2",
            (1, 1, false).into(),
            TopicStatus::new(
                TopicResolution::Pending,
                vec![],
                "waiting for live spus".to_owned(),
            ),
        );
        assert_eq!(topic2.status.is_resolution_pending(), true);

        // resolution: Ok
        let topic3 = DefaultTopicMd::new(
            "Topic-3",
            (2, 2, false).into(),
            TopicStatus::new(
                TopicResolution::Provisioned,
                vec![vec![0, 1], vec![1, 2]],
                "".to_owned(),
            ),
        );
        assert_eq!(topic3.status.is_resolution_provisioned(), true);

        // resolution: Inconsistent
        let topic4 = DefaultTopicMd::new(
            "Topic-4",
            (2, 2, false).into(),
            TopicStatus::new(
                TopicResolution::InsufficientResources,
                vec![vec![0], vec![1]],
                "".to_owned(),
            ),
        );

        topics.insert(topic1).await;
        topics.insert(topic2).await;
        topics.insert(topic3).await;
        topics.insert(topic4).await;

        let expected = vec![String::from("Topic-2"), String::from("Topic-4")];
        let mut pending_state_names: Vec<String> = vec![];

        for topic in topics.read().await.values() {
            if topic.status.need_replica_map_recal() {
                pending_state_names.push(topic.key_owned());
            }
        }

        assert_eq!(pending_state_names, expected);
        Ok(())
    }

    #[test_async]
    async fn test_update_topic_status_with_other_error_topic_not_found() -> Result<(), ()> {
        let topics = DefaultTopicLocalStore::default();

        let topic1 = DefaultTopicMd::new("Topic-1", (1, 1, false).into(), TopicStatus::default());
        topics.insert(topic1).await;

        let topic2 = DefaultTopicMd::new(
            "Topic-2",
            (2, 2, false).into(),
            TopicStatus::new(
                TopicResolution::Provisioned,
                vec![vec![0, 1], vec![1, 2]],
                "".to_owned(),
            ),
        );

        // test: update_status (returns error)
        let res = topics
            .update_status(topic2.key(), topic2.status.clone())
            .await;
        assert_eq!(
            format!("{}", res.unwrap_err()),
            "Topic 'Topic-2': not found, cannot update"
        );
        Ok(())
    }

    #[test_async]
    async fn test_update_topic_status_successful() -> Result<(), ()> {
        let topics = DefaultTopicLocalStore::default();
        let topic1 = DefaultTopicMd::new("Topic-1", (2, 2, false).into(), TopicStatus::default());
        topics.insert(topic1).await;

        let updated_topic = DefaultTopicMd::new(
            "Topic-1",
            (2, 2, false).into(),
            TopicStatus::new(
                TopicResolution::Provisioned,
                vec![vec![0, 1], vec![1, 2]],
                "".to_owned(),
            ),
        );

        // run test
        let res = topics
            .update_status(updated_topic.key(), updated_topic.status.clone())
            .await;
        assert!(res.is_ok());

        let topic = topics.topic("Topic-1").await;
        assert_eq!(topic.is_some(), true);

        assert_eq!(topic.unwrap(), updated_topic);
        Ok(())
    }

}