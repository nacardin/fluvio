use futures::channel::mpsc::Receiver;
use async_trait::async_trait;

use flv_util::actions::Actions;
use flv_metadata::k8::metadata::ObjectMeta;
use flv_metadata::spu::SpuSpec;
use flv_metadata::topic::TopicSpec;
use flv_metadata::partition::PartitionSpec;
use flv_metadata::store::actions::*;

use crate::ScServerError;



/// Update the world state
#[async_trait]
pub trait WSUpdateService {
    /// update the spu
    async fn update_spu(&self, ws_actions: WSAction<SpuSpec,ObjectMeta>) -> Result<(), ScServerError>;

    /// update the topic
    async fn update_topic(&self, ws_actions: WSAction<TopicSpec,ObjectMeta>) -> Result<(), ScServerError>;

    /// update the partition
    async fn update_partition(
        &self,
        ws_actions: WSAction<PartitionSpec,ObjectMeta>,
    ) -> Result<(), ScServerError>;
}

pub type WSChangeChannel<S> = Receiver<Actions<LSChange<S,ObjectMeta>>>;

pub trait WSChangeDispatcher {
    fn create_spu_channel(&mut self) -> WSChangeChannel<SpuSpec>;

    fn create_topic_channel(&mut self) -> WSChangeChannel<TopicSpec>;

    fn create_partition_channel(&mut self) -> WSChangeChannel<PartitionSpec>;
}
