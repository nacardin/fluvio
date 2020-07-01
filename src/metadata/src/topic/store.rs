use std::collections::BTreeMap;
use std::fmt;

use log::trace;
use log::debug;
use log::warn;


use flv_types::ReplicaMap;
use flv_metadata::topic::*;
use flv_metadata::partition::ReplicaKey;


use crate::parti
use super::spu::*;
use super::*;


pub type TopicLocalStore<C> = LocalStore<TopicSpec,C>;
pub type DefaultTopicMd = TopicMetadata<String>;
pub type DefaultTopicLocalStore = TopicLocalStore<String>;