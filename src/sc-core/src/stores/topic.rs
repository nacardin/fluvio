//!
//! # Topic & Topics Metadata
//!
//! Topic metadata information cached on SC.
//!
//! # Remarks
//! Topic Status uses TopicResolution to reflect the state of the replica map:
//!     Ok,           // replica map has been generated, topic is operational
//!     Pending,      // not enough SPUs to generate "replica map"
//!     Inconsistent, // use change spec parameters, which is not supported
//!     InvalidConfig, // invalid configuration parameters provided
//!
use std::collections::BTreeMap;
use std::fmt;

use log::trace;
use log::debug;
use log::warn;
use rand::thread_rng;
use rand::Rng;

use flv_types::ReplicaMap;
use flv_metadata::topic::*;
use flv_metadata::partition::ReplicaKey;
use flv_metadata::k8::metadata::ObjectMeta;

use super::partition::*;
use super::spu::*;
use super::*;


pub type TopicLocalStore<C> = LocalStore<TopicSpec,C>;
pub type DefaultTopicMd = TopicMetadata<String>;
pub type DefaultTopicLocalStore = TopicLocalStore<String>;



impl K8ExtendedSpec for TopicSpec {
    type K8Spec = Self;
    type K8Status = Self::Status;
}



/// values for next state
#[derive(Default, Debug)]
pub struct TopicNextState {
    pub resolution: TopicResolution,
    pub reason: String,
    pub replica_map: ReplicaMap,
    pub partitions: Vec<K8PartitionMd>,
}

impl fmt::Display for TopicNextState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.resolution)
    }
}

impl From<(TopicResolution, String)> for TopicNextState {
    fn from(val: (TopicResolution, String)) -> Self {
        let (resolution, reason) = val;
        Self {
            resolution,
            reason,
            ..Default::default()
        }
    }
}

impl From<((TopicResolution, String), ReplicaMap)> for TopicNextState {
    fn from(val: ((TopicResolution, String), ReplicaMap)) -> Self {
        let ((resolution, reason), replica_map) = val;
        Self {
            resolution,
            reason,
            replica_map,
            ..Default::default()
        }
    }
}

impl From<((TopicResolution, String), Vec<K8SpuMetadata>)> for TopicNextState {
    fn from(val: ((TopicResolution, String), Vec<K8SpuMetadata>)) -> Self {
        let ((resolution, reason), partitions) = val;
        Self {
            resolution,
            reason,
            partitions,
            ..Default::default()
        }
    }
}





impl K8TopicMd {
    pub fn same_next_state(&self) -> TopicNextState {
        TopicNextState {
            resolution: self.status.resolution.clone(),
            ..Default::default()
        }
    }

    /// update our state with next state, return remaining partition kv changes
    pub fn apply_next_state(&mut self, next_state: TopicNextState) -> Vec<K8PartitionMd> {
        self.status.resolution = next_state.resolution;
        self.status.reason = next_state.reason;
        if next_state.replica_map.len() > 0 {
            self.status.set_replica_map(next_state.replica_map);
        }
        next_state.partitions
    }

    /// based on our current state, compute what should be next state
    pub async fn compute_next_state(
        &self,
        spu_store: &K8SpuLocalStore,
        partition_store: &K8PartitionLocalStore,
    ) -> TopicNextState {
        match self.spec() {
            // Computed Topic
            TopicSpec::Computed(ref param) => match self.status.resolution {
                TopicResolution::Init | TopicResolution::InvalidConfig => {
                    self.validate_computed_topic_parameters(param)
                }
                TopicResolution::Pending | TopicResolution::InsufficientResources => {
                    let mut next_state = self.generate_replica_map(spu_store, param).await;
                    if next_state.resolution == TopicResolution::Provisioned {
                        debug!(
                            "Topic: {} replica generate successfull, status is provisioned",
                            self.key()
                        );
                        next_state.partitions = self.create_new_partitions(partition_store).await;
                        next_state
                    } else {
                        next_state
                    }
                }
                _ => {
                    debug!(
                        "topic: {} resolution: {:#?} ignoring",
                        self.key, self.status.resolution
                    );
                    let mut next_state = self.same_next_state();
                    if next_state.resolution == TopicResolution::Provisioned {
                        next_state.partitions = self.create_new_partitions(partition_store).await;
                        next_state
                    } else {
                        next_state
                    }
                }
            },

            // Assign Topic
            TopicSpec::Assigned(ref partition_map) => match self.status.resolution {
                TopicResolution::Init | TopicResolution::InvalidConfig => {
                    self.validate_assigned_topic_parameters(partition_map)
                }
                TopicResolution::Pending | TopicResolution::InsufficientResources => {
                    let mut next_state = self
                        .update_replica_map_for_assigned_topic(partition_map, spu_store)
                        .await;
                    if next_state.resolution == TopicResolution::Provisioned {
                        next_state.partitions = self.create_new_partitions(partition_store).await;
                        next_state
                    } else {
                        next_state
                    }
                }
                _ => {
                    debug!(
                        "assigned topic: {} resolution: {:#?} ignoring",
                        self.key, self.status.resolution
                    );
                    let mut next_state = self.same_next_state();
                    if next_state.resolution == TopicResolution::Provisioned {
                        next_state.partitions = self.create_new_partitions(partition_store).await;
                        next_state
                    } else {
                        next_state
                    }
                }
            },
        }
    }

    ///
    /// Validate computed topic spec parameters and update topic status
    ///  * error is passed to the topic reason.
    ///
    pub fn validate_computed_topic_parameters(&self, param: &TopicReplicaParam) -> TopicNextState {
        if let Err(err) = TopicSpec::valid_partition(&param.partitions) {
            warn!("topic: {} partition config is invalid", self.key());
            TopicStatus::next_resolution_invalid_config(&err.to_string()).into()
        } else if let Err(err) = TopicSpec::valid_replication_factor(&param.replication_factor) {
            warn!("topic: {} replication config is invalid", self.key());
            TopicStatus::next_resolution_invalid_config(&err.to_string()).into()
        } else {
            debug!(
                "topic: {} config is valid, transition to pending",
                self.key()
            );
            TopicStatus::next_resolution_pending().into()
        }
    }

    ///
    /// Validate assigned topic spec parameters and update topic status
    ///  * error is passed to the topic reason.
    ///
    pub fn validate_assigned_topic_parameters(
        &self,
        partition_map: &PartitionMaps,
    ) -> TopicNextState {
        if let Err(err) = partition_map.valid_partition_map() {
            TopicStatus::next_resolution_invalid_config(&err.to_string()).into()
        } else {
            TopicStatus::next_resolution_pending().into()
        }
    }

    ///
    /// Genereate Replica Map if there are enough online spus
    ///  * returns a replica map or a reason for the failure
    ///  * fatal error sare configuration errors and are not recovarable
    ///
    pub async fn generate_replica_map(
        &self,
        spus: &K8SpuLocalStore,
        param: &TopicReplicaParam,
    ) -> TopicNextState {
        let spu_count = spus.count().await;
        if spu_count < param.replication_factor {
            trace!(
                "topic '{}' - R-MAP needs {:?} online spus, found {:?}",
                self.key,
                param.replication_factor,
                spu_count
            );

            let reason = format!("need {} more SPU", param.replication_factor - spu_count);
            TopicStatus::set_resolution_no_resource(reason).into()
        } else {
            let replica_map = generate_replica_map_for_topic(spus, param, None).await;
            if replica_map.len() > 0 {
                (TopicStatus::next_resolution_provisoned(), replica_map).into()
            } else {
                let reason = "empty replica map";
                TopicStatus::set_resolution_no_resource(reason.to_owned()).into()
            }
        }
    }

    /// create partition children if it doesn't exists
    pub async fn create_new_partitions(
        &self,
        partition_store: &K8PartitionLocalStore,
    ) -> Vec<K8PartitionMd> {
        let parent_kv_ctx = self.kv_ctx.make_parent_ctx();

        let mut partitions = vec![];
        for (idx, replicas) in self.status.replica_map.iter() {
            let replica_key = ReplicaKey::new(self.key(), *idx);
            debug!("Topic: {} creating partition: {}", self.key(), replica_key);
            if !partition_store.contains_key(&replica_key).await {
                partitions.push(
                    K8PartitionMd::with_spec(replica_key, replicas.clone().into())
                        .with_kv_ctx(parent_kv_ctx.clone()),
                )
            }
        }
        partitions
    }

    ///
    /// Compare assigned SPUs versus local SPUs. If all assigned SPUs are live,
    /// update topic status to ok. otherwise, mark as waiting for live SPUs
    ///
    pub async fn update_replica_map_for_assigned_topic(
        &self,
        partition_maps: &PartitionMaps,
        spu_store: &K8SpuLocalStore,
    ) -> TopicNextState {
        let partition_map_spus = partition_maps.unique_spus_in_partition_map();
        let spus_id = spu_store.spu_ids_for_replica().await;

        // ensure spu exists
        for spu in &partition_map_spus {
            if !spus_id.contains(spu) {
                return TopicStatus::next_resolution_invalid_config(format!(
                    "invalid spu id: {}",
                    spu
                ))
                .into();
            }
        }

        let replica_map = partition_maps.partition_map_to_replica_map();
        if replica_map.len() == 0 {
            TopicStatus::next_resolution_invalid_config("invalid replica map".to_owned()).into()
        } else {
            (TopicStatus::next_resolution_provisoned(), replica_map).into()
        }
    }
}

///
/// Generate replica map for a specific topic
///
pub async fn generate_replica_map_for_topic(
    spus: &K8SpuLocalStore,
    param: &TopicReplicaParam,
    from_index: Option<i32>,
) -> ReplicaMap {
    let in_rack_count = spus.spus_in_rack_count().await;
    let start_index = from_index.unwrap_or(-1);

    // generate partition map (with our without rack assignment)
    if param.ignore_rack_assignment || in_rack_count == 0 {
        generate_partitions_without_rack(&spus, &param, start_index).await
    } else {
        generate_partitions_with_rack_assignment(&spus, &param, start_index).await
    }
}

///
/// Generate partitions on spus that have been assigned to racks
///
async fn generate_partitions_with_rack_assignment(
    spus: &K8SpuLocalStore,
    param: &TopicReplicaParam,
    start_index: i32,
) -> ReplicaMap {
    let mut partition_map = BTreeMap::new();
    let rack_map = SpuLocalStore::live_spu_rack_map_sorted(&spus).await;
    let spu_list = SpuLocalStore::online_spus_in_rack(&rack_map);
    let spu_cnt = spus.online_spu_count().await;

    let s_idx = if start_index >= 0 {
        start_index
    } else {
        thread_rng().gen_range(0, spu_cnt)
    };

    for p_idx in 0..param.partitions {
        let mut replicas: Vec<i32> = vec![];
        for r_idx in 0..param.replication_factor {
            let spu_idx = ((s_idx + p_idx + r_idx) % spu_cnt) as usize;
            replicas.push(spu_list[spu_idx]);
        }
        partition_map.insert(p_idx, replicas);
    }

    partition_map
}

///
/// Generate partitions without taking rack assignments into consideration
///
async fn generate_partitions_without_rack(
    spus: &K8SpuLocalStore,
    param: &TopicReplicaParam,
    start_index: i32,
) -> ReplicaMap {
    let mut partition_map = BTreeMap::new();
    let spu_cnt = spus.spu_used_for_replica().await;
    let spu_ids = spus.spu_ids_for_replica().await;

    let s_idx = if start_index >= 0 {
        start_index
    } else {
        thread_rng().gen_range(0, spu_cnt)
    };

    let gap_max = spu_cnt - param.replication_factor + 1;
    for p_idx in 0..param.partitions {
        let mut replicas: Vec<i32> = vec![];
        let gap_cnt = ((s_idx + p_idx) / spu_cnt) % gap_max;
        for r_idx in 0..param.replication_factor {
            let gap = if r_idx != 0 { gap_cnt } else { 0 };
            let spu_idx = ((s_idx + p_idx + r_idx + gap) % spu_cnt) as usize;
            replicas.push(spu_ids[spu_idx]);
        }
        partition_map.insert(p_idx, replicas);
    }

    partition_map
}



// -----------------------------------
// Topics - Implementation
// -----------------------------------

impl <C>TopicLocalStore<C> {
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

//
// Unit Tests
//
#[cfg(test)]

mod test {
    use flv_metadata::topic::{TopicResolution, TopicStatus};
    use flv_future_aio::test_async;

    use super::{DefaultTopicMd, DefaultTopicLocalStore};

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

        // resulution: Pending
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

//
// Unit Tests
//
#[cfg(test)]
pub mod replica_map_test {

    use std::collections::BTreeMap;

    use flv_future_aio::test_async;

    use super::SpuLocalStore;
    use super::generate_replica_map_for_topic;

    #[test_async]
    async fn generate_replica_map_for_topic_1x_replicas_no_rack() -> Result<(), ()> {
        let spus: SpuLocalStore = vec![
            (0, true, None),
            (1, true, None),
            (2, true, None),
            (4, true, None),
            (5000, true, None),
        ]
        .into();

        assert_eq!(spus.online_spu_count().await, 5);

        // test 4 partitions, 1 replicas - index 8
        let param = (4, 1, false).into();
        let map_1xi = generate_replica_map_for_topic(&spus, &param, Some(8)).await;
        let mut map_1xi_expected = BTreeMap::new();
        map_1xi_expected.insert(0, vec![4]);
        map_1xi_expected.insert(1, vec![5000]);
        map_1xi_expected.insert(2, vec![0]);
        map_1xi_expected.insert(3, vec![1]);
        assert_eq!(map_1xi, map_1xi_expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_2x_replicas_no_rack() -> Result<(), ()> {
        let spus = vec![
            (0, true, None),
            (1, true, None),
            (2, true, None),
            (3, true, None),
            (4, true, None),
        ]
        .into();

        // test 4 partitions, 2 replicas - index 3
        let param = (4, 2, false).into();
        let map_2xi = generate_replica_map_for_topic(&spus, &param, Some(3)).await;
        let mut map_2xi_expected = BTreeMap::new();
        map_2xi_expected.insert(0, vec![3, 4]);
        map_2xi_expected.insert(1, vec![4, 0]);
        map_2xi_expected.insert(2, vec![0, 2]);
        map_2xi_expected.insert(3, vec![1, 3]);
        assert_eq!(map_2xi, map_2xi_expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_3x_replicas_no_rack() -> Result<(), ()> {
        let spus = vec![
            (0, true, None),
            (1, true, None),
            (2, true, None),
            (3, true, None),
            (4, true, None),
        ]
        .into();

        // test 21 partitions, 3 replicas - index 0
        let param = (21, 3, false).into();
        let map_3x = generate_replica_map_for_topic(&spus, &param, Some(0)).await;
        let mut map_3x_expected = BTreeMap::new();
        map_3x_expected.insert(0, vec![0, 1, 2]);
        map_3x_expected.insert(1, vec![1, 2, 3]);
        map_3x_expected.insert(2, vec![2, 3, 4]);
        map_3x_expected.insert(3, vec![3, 4, 0]);
        map_3x_expected.insert(4, vec![4, 0, 1]);
        map_3x_expected.insert(5, vec![0, 2, 3]);
        map_3x_expected.insert(6, vec![1, 3, 4]);
        map_3x_expected.insert(7, vec![2, 4, 0]);
        map_3x_expected.insert(8, vec![3, 0, 1]);
        map_3x_expected.insert(9, vec![4, 1, 2]);
        map_3x_expected.insert(10, vec![0, 3, 4]);
        map_3x_expected.insert(11, vec![1, 4, 0]);
        map_3x_expected.insert(12, vec![2, 0, 1]);
        map_3x_expected.insert(13, vec![3, 1, 2]);
        map_3x_expected.insert(14, vec![4, 2, 3]);
        map_3x_expected.insert(15, vec![0, 1, 2]);
        map_3x_expected.insert(16, vec![1, 2, 3]);
        map_3x_expected.insert(17, vec![2, 3, 4]);
        map_3x_expected.insert(18, vec![3, 4, 0]);
        map_3x_expected.insert(19, vec![4, 0, 1]);
        map_3x_expected.insert(20, vec![0, 2, 3]);
        assert_eq!(map_3x, map_3x_expected);

        // test 4 partitions, 3 replicas - index 12
        let param = (4, 3, false).into();
        let map_3xi = generate_replica_map_for_topic(&spus, &param, Some(12)).await;
        let mut map_3xi_expected = BTreeMap::new();
        map_3xi_expected.insert(0, vec![2, 0, 1]);
        map_3xi_expected.insert(1, vec![3, 1, 2]);
        map_3xi_expected.insert(2, vec![4, 2, 3]);
        map_3xi_expected.insert(3, vec![0, 1, 2]);
        assert_eq!(map_3xi, map_3xi_expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_4x_replicas_no_rack() -> Result<(), ()> {
        let spus = vec![
            (0, true, None),
            (1, true, None),
            (2, true, None),
            (3, true, None),
            (4, true, None),
        ]
        .into();

        // test 4 partitions, 4 replicas - index 10
        let param = (4, 4, false).into();
        let map_4xi = generate_replica_map_for_topic(&spus, &param, Some(10)).await;
        let mut map_4xi_expected = BTreeMap::new();
        map_4xi_expected.insert(0, vec![0, 1, 2, 3]);
        map_4xi_expected.insert(1, vec![1, 2, 3, 4]);
        map_4xi_expected.insert(2, vec![2, 3, 4, 0]);
        map_4xi_expected.insert(3, vec![3, 4, 0, 1]);
        assert_eq!(map_4xi, map_4xi_expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_5x_replicas_no_rack() -> Result<(), ()> {
        let spus = vec![
            (0, true, None),
            (1, true, None),
            (3, true, None),
            (4, true, None),
            (5002, true, None),
        ]
        .into();

        // test 4 partitions, 5 replicas - index 14
        let param = (4, 5, false).into();
        let map_5xi = generate_replica_map_for_topic(&spus, &param, Some(14)).await;
        let mut map_5xi_expected = BTreeMap::new();
        map_5xi_expected.insert(0, vec![5002, 0, 1, 3, 4]);
        map_5xi_expected.insert(1, vec![0, 1, 3, 4, 5002]);
        map_5xi_expected.insert(2, vec![1, 3, 4, 5002, 0]);
        map_5xi_expected.insert(3, vec![3, 4, 5002, 0, 1]);
        assert_eq!(map_5xi, map_5xi_expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_6_part_3_rep_6_brk_3_rak() -> Result<(), ()> {
        let r1 = String::from("r1");
        let r2 = String::from("r2");
        let r3 = String::from("r3");

        let spus = vec![
            (0, true, Some(r1.clone())),
            (1, true, Some(r2.clone())),
            (2, true, Some(r2.clone())),
            (3, true, Some(r3.clone())),
            (4, true, Some(r3.clone())),
            (5, true, Some(r3.clone())),
        ]
        .into();

        // Compute & compare with result
        let param = (6, 3, false).into();
        let computed = generate_replica_map_for_topic(&spus, &param, Some(0)).await;
        let mut expected = BTreeMap::new();
        expected.insert(0, vec![3, 2, 0]);
        expected.insert(1, vec![2, 0, 4]);
        expected.insert(2, vec![0, 4, 1]);
        expected.insert(3, vec![4, 1, 5]);
        expected.insert(4, vec![1, 5, 3]);
        expected.insert(5, vec![5, 3, 2]);

        assert_eq!(computed, expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_12_part_4_rep_11_brk_4_rak() -> Result<(), ()> {
        let r1 = String::from("r1");
        let r2 = String::from("r2");
        let r3 = String::from("r3");
        let r4 = String::from("r4");

        let spus = vec![
            (0, true, Some(r1.clone())),
            (1, true, Some(r1.clone())),
            (2, true, Some(r1.clone())),
            (3, true, Some(r2.clone())),
            (4, true, Some(r2.clone())),
            (5, true, Some(r2.clone())),
            (6, true, Some(r3.clone())),
            (7, true, Some(r3.clone())),
            (8, true, Some(r3.clone())),
            (9, true, Some(r4.clone())),
            (10, true, Some(r4.clone())),
            (11, true, Some(r4.clone())),
        ]
        .into();

        // Compute & compare with result
        let param = (12, 4, false).into();
        let computed = generate_replica_map_for_topic(&spus, &param, Some(0)).await;
        let mut expected = BTreeMap::new();
        expected.insert(0, vec![0, 4, 8, 9]);
        expected.insert(1, vec![4, 8, 9, 1]);
        expected.insert(2, vec![8, 9, 1, 5]);
        expected.insert(3, vec![9, 1, 5, 6]);
        expected.insert(4, vec![1, 5, 6, 10]);
        expected.insert(5, vec![5, 6, 10, 2]);
        expected.insert(6, vec![6, 10, 2, 3]);
        expected.insert(7, vec![10, 2, 3, 7]);
        expected.insert(8, vec![2, 3, 7, 11]);
        expected.insert(9, vec![3, 7, 11, 0]);
        expected.insert(10, vec![7, 11, 0, 4]);
        expected.insert(11, vec![11, 0, 4, 8]);

        assert_eq!(computed, expected);
        Ok(())
    }

    #[test_async]
    async fn generate_replica_map_for_topic_9_part_3_rep_9_brk_3_rak() -> Result<(), ()> {
        let r1 = String::from("r1");
        let r2 = String::from("r2");
        let r3 = String::from("r3");

        let spus = vec![
            (0, true, Some(r1.clone())),
            (1, true, Some(r1.clone())),
            (2, true, Some(r1.clone())),
            (3, true, Some(r2.clone())),
            (4, true, Some(r2.clone())),
            (5, true, Some(r2.clone())),
            (6, true, Some(r3.clone())),
            (7, true, Some(r3.clone())),
            (8, true, Some(r3.clone())),
        ]
        .into();

        // test 9 partitions, 3 replicas - index 0
        let param = (9, 3, false).into();
        let computed = generate_replica_map_for_topic(&spus, &param, Some(0)).await;
        let mut expected = BTreeMap::new();
        expected.insert(0, vec![0, 4, 8]);
        expected.insert(1, vec![4, 8, 1]);
        expected.insert(2, vec![8, 1, 5]);
        expected.insert(3, vec![1, 5, 6]);
        expected.insert(4, vec![5, 6, 2]);
        expected.insert(5, vec![6, 2, 3]);
        expected.insert(6, vec![2, 3, 7]);
        expected.insert(7, vec![3, 7, 0]);
        expected.insert(8, vec![7, 0, 4]);

        assert_eq!(computed, expected);
        Ok(())
    }
}
