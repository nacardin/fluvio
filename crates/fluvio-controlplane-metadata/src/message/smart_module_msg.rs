#![allow(clippy::assign_op_pattern)]

//!
//! # Smart Module Messages
//!
//! Smart Modules are sent from SC to all SPUs.
//!
use std::fmt;

use dataplane::core::{Encoder, Decoder};
use fluvio_stream_model::core::MetadataItem;
use fluvio_stream_model::store::MetadataStoreObject;

use crate::smartmodule::SmartModuleSpec;

use super::MsgType;
use super::Message;

#[derive(Decoder, Encoder, Debug, PartialEq, Clone, Default)]
pub struct SmartModule {
    name: String
}


impl fmt::Display for SmartModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: SmartModule")
    }
}

impl<C> From<MetadataStoreObject<SmartModuleSpec, C>> for SmartModule
where
    C: MetadataItem,
{
    fn from(item: MetadataStoreObject<SmartModuleSpec, C>) -> Self {
        Self {
            name: item.key
        }
    }
}

pub type SmartModuleMsg = Message<SmartModule>;

#[derive(Decoder, Encoder, Debug, PartialEq, Clone, Default)]
pub struct SmartModuleMsgs {
    pub messages: Vec<SmartModuleMsg>,
}

impl fmt::Display for SmartModuleMsgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for sm in &self.messages {
            write!(f, "{},", sm)?;
        }
        write!(f, "]")
    }
}

impl SmartModuleMsgs {
    pub fn new(sm_msgs: Vec<SmartModuleMsg>) -> Self {
        SmartModuleMsgs {
            messages: sm_msgs,
        }
    }

    pub fn push(&mut self, msg: SmartModuleMsg) {
        self.messages.push(msg);
    }
}

impl SmartModuleMsg {
    pub fn create_delete_msg(name: String) -> Self {
        SmartModuleMsg {
            header: MsgType::DELETE,
            content: SmartModule {
                name
            },
        }
    }
}