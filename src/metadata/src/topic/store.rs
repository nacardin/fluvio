

use crate::store::*;
use super::*;

pub type TopicMetadata<C> = MetadataStoreObject<TopicSpec,C>;
pub type TopicLocalStore<C> = LocalStore<TopicSpec,C>;
pub type DefaultTopicMd = TopicMetadata<String>;
pub type DefaultTopicLocalStore = TopicLocalStore<String>;