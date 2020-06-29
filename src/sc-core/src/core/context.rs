//!
//! # Streaming Coordinator Metadata
//!
//! Metadata stores a copy of the data from KV store in local memory.
//!
use std::sync::Arc;


use flv_future_aio::sync::broadcast::*;

use crate::config::ScConfig;
use crate::stores::spu::*;
use crate::stores::partition::*;
use crate::stores::topic::*;
use crate::stores::spg::SpuGroupLocalStore;
use crate::controllers::ClientNotification;

pub type SharedContext = Arc<Context>;

#[derive(Debug)]
pub struct Context {
    client_channel: Channel<ClientNotification>,
    spus: SharedSpuLocalStore,
    partitions: Arc<PartitionLocalStore>,
    topics: Arc<TopicLocalStore>,
    spgs: Arc<SpuGroupLocalStore>,
    config: ScConfig,
}

// -----------------------------------
// ScMetadata - Implementation
// -----------------------------------

impl Context {
    pub fn shared_metadata(config: ScConfig) -> Arc<Self> {
        Arc::new(Self::new(config))
    }

    /// private function to provision metadata
    fn new(config: ScConfig) -> Self {

        Self {
            client_channel: Channel::new(100),
            spus: SpuLocalStore::new_shared(),
            partitions: PartitionLocalStore::new_shared(),
            topics: TopicLocalStore::new_shared(),
            spgs: SpuGroupLocalStore::new_shared(),
            config: config,
        }
    }

    /// reference to spus
    pub fn spus(&self) -> &SharedSpuLocalStore {
        &self.spus
    }

    pub fn owned_spus(&self) -> SharedSpuLocalStore {
        self.spus().clone()
    }

    /// reference to partitions
    pub fn partitions(&self) -> &Arc<PartitionLocalStore> {
        &self.partitions
    }

    /// reference to topics
    pub fn topics(&self) -> &Arc<TopicLocalStore> {
        &self.topics
    }

    pub fn spgs(&self) -> &Arc<SpuGroupLocalStore> {
        &self.spgs
    }

    /// reference to config
    pub fn config(&self) -> &ScConfig {
        &self.config
    }

    /// create new client subscriber
    pub fn new_client_subscriber(&self) -> Receiver<ClientNotification> {
        self.client_channel.receiver()
    }

    pub fn new_client_sender(&self) -> Sender<ClientNotification> {
        self.client_channel.sender()
    }

    /// format metadata cache into a table string
    #[allow(dead_code)]
    pub async fn table_fmt(&self) -> String {
        let mut table = String::new();
        let newline = format!("\n");

        table.push_str(&self.spus.table_fmt().await);
        table.push_str(&newline);
        table.push_str(&self.topics.table_fmt().await);
        table.push_str(&newline);
        table.push_str(&self.partitions.table_fmt().await);
        table
    }
}
