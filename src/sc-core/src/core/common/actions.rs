use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;

use flv_metadata::core::K8ExtendedSpec; 
use flv_metadata::core::Spec; 
use crate::stores::*;

/// Represents changes in Local State
#[derive(Debug, PartialEq, Clone)]
pub enum LSChange<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
    S::Status: PartialEq,
{
    Add(KVObject<S>),
    Mod(KVObject<S>, KVObject<S>), // new, old
    Delete(KVObject<S>),
}

impl<S> fmt::Display for LSChange<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
    S::IndexKey: Display,
    S::Status:  PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add(add) => write!(f, "{} Add: {}", S::LABEL, add.key()),
            Self::Mod(update, _) => write!(f, "{} Mod: {}", S::LABEL, update.key()),
            Self::Delete(del) => write!(f, "{} Delete: {}", S::LABEL, del.key()),
        }
    }
}

impl<S> LSChange<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
    S::Status: PartialEq,
{
    pub fn add<K>(value: K) -> Self
    where
        K: Into<KVObject<S>>,
    {
        LSChange::Add(value.into())
    }

    pub fn update(new: KVObject<S>, old: KVObject<S>) -> Self {
        LSChange::Mod(new, old)
    }

    pub fn delete(value: KVObject<S>) -> Self {
        LSChange::Delete(value)
    }
}

/// Actions to update World States
#[derive(Debug, PartialEq, Clone)]
pub enum WSAction<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
    S::IndexKey: PartialEq,
    S::Status:  PartialEq,
{
    Add(KVObject<S>),
    UpdateStatus(KVObject<S>), 
    UpdateSpec(KVObject<S>),  
    Delete(S::IndexKey),
}
