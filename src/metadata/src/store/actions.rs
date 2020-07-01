use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;


use crate::core::*;
use crate::store::*;

/// Represents changes in Local State
#[derive(Debug, PartialEq, Clone)]
pub enum LSChange<S,C>
where
    S: Spec,
    S::Status: PartialEq,
    C: MetadataItem
{
    Add(MetadataStoreObject<S,C>),
    Mod(MetadataStoreObject<S,C>, MetadataStoreObject<S,C>), // new, old
    Delete(MetadataStoreObject<S,C>),
}

impl<S,C> fmt::Display for LSChange<S,C>
where
    S: Spec,
    S::IndexKey: Display,
    S::Status: PartialEq,
    C: MetadataItem
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add(add) => write!(f, "{} Add: {}", S::LABEL, add.key()),
            Self::Mod(update, _) => write!(f, "{} Mod: {}", S::LABEL, update.key()),
            Self::Delete(del) => write!(f, "{} Delete: {}", S::LABEL, del.key()),
        }
    }
}

impl<S,C> LSChange<S,C>
where
    S: Spec,
    S::Status: PartialEq,
    C: MetadataItem
{
    pub fn add<K>(value: K) -> Self
    where
        K: Into<MetadataStoreObject<S,C>>,
    {
        LSChange::Add(value.into())
    }

    pub fn update(new: MetadataStoreObject<S,C>, old: MetadataStoreObject<S,C>) -> Self {
        LSChange::Mod(new, old)
    }

    pub fn delete(value: MetadataStoreObject<S,C>) -> Self {
        LSChange::Delete(value)
    }
}

/// Actions to update World States
#[derive(Debug, PartialEq, Clone)]
pub enum WSAction<S,C>
where
    S: Spec,
    S::IndexKey: PartialEq,
    S::Status: PartialEq,
    C: MetadataItem
{
    Add(MetadataStoreObject<S,C>),
    UpdateStatus(MetadataStoreObject<S,C>),
    UpdateSpec(MetadataStoreObject<S,C>),
    Delete(S::IndexKey),
}
