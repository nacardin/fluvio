use std::sync::Arc;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::collections::BTreeMap;

use flv_future_aio::sync::RwLockReadGuard;
use flv_future_aio::sync::RwLockWriteGuard;


use crate::core::Spec;
use super::SimpleConcurrentBTreeMap;
use super::*;

pub enum CheckExist {
    // doesn't exist
    None,
    // exists, but same value
    Same,
    // exists, but different
    Different,
}

/// Local state in memory
#[derive(Debug)]
pub struct LocalStore<S,C>(SimpleConcurrentBTreeMap<S::IndexKey, MetadataStoreObject<S,C>>)
where
    S: Spec,
    C: Clone + Debug;

impl<S,C> Default for LocalStore<S,C>
where
    S: Spec,
    C: Clone + Debug
{
    fn default() -> Self {
        Self(SimpleConcurrentBTreeMap::new())
    }
}

impl<S,C> LocalStore<S,C>
where
    S: Spec,
    C: Clone + Debug
{
    pub fn bulk_new(objects: Vec<MetadataStoreObject<S,C>>) -> Self {
        let mut map = BTreeMap::new();
        for obj in objects {
            map.insert(obj.key.clone(), obj);
        }
        Self(SimpleConcurrentBTreeMap::new_with_map(map))
    }

    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// read access
    #[inline(always)]
    pub async fn read<'a>(
        &'a self,
    ) -> RwLockReadGuard<'a, BTreeMap<S::IndexKey, MetadataStoreObject<S,C>>> {
        self.0.read().await
    }

    #[inline(always)]
    pub async fn write<'a>(
        &'a self,
    ) -> RwLockWriteGuard<'a, BTreeMap<S::IndexKey, MetadataStoreObject<S,C>>> {
        self.0.write().await
    }

    pub async fn insert(&self, value: MetadataStoreObject<S,C>) -> Option<MetadataStoreObject<S,C>> {
        self.write().await.insert(value.key_owned(), value)
    }

    pub fn try_insert(&self, value: MetadataStoreObject<S,C>) -> Option<MetadataStoreObject<S,C>> {
        self.0.try_write().unwrap().insert(value.key_owned(), value)
    }

    /// get copy of the value ref by key
    pub async fn value<K: ?Sized>(&self, key: &K) -> Option<MetadataStoreObject<S,C>>
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        match self.read().await.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    /// get copy of the spec ref by key
    pub async fn spec<K: ?Sized>(&self, key: &K) -> Option<S>
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        match self.read().await.get(key) {
            Some(value) => Some(value.spec.clone()),
            None => None,
        }
    }

    pub async fn find_and_do<K, F>(&self, key: &K, mut func: F) -> Option<()>
    where
        F: FnMut(&'_ MetadataStoreObject<S,C>),
        K: Ord,
        S::IndexKey: Borrow<K>,
    {
        if let Some(value) = self.read().await.get(key) {
            func(value);
            Some(())
        } else {
            None
        }
    }

    pub async fn contains_key<K: ?Sized>(&self, key: &K) -> bool
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        self.read().await.contains_key(key)
    }

    pub async fn remove<K: ?Sized>(&self, key: &K) -> Option<MetadataStoreObject<S,C>>
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        self.write().await.remove(key)
    }

    pub async fn count(&self) -> i32 {
        self.read().await.len() as i32
    }

    /// get copy of keys
    pub async fn clone_keys(&self) -> Vec<S::IndexKey> {
        self.read().await.keys().cloned().collect()
    }

    pub async fn clone_values(&self) -> Vec<MetadataStoreObject<S,C>> {
        self.read().await.values().cloned().collect()
    }

    pub async fn clone_specs(&self) -> Vec<S> {
        self.read()
            .await
            .values()
            .map(|kv| kv.spec.clone())
            .collect()
    }

    /// update status
    pub async fn update_status<K: ?Sized>(&self, key: &K, status: S::Status) -> Result<(), IoError>
    where
        S::IndexKey: Borrow<K>,
        K: Display + Ord,
    {
        if let Some(old_kv) = self.write().await.get_mut(key) {
            old_kv.status = status;
            Ok(())
        } else {
            Err(IoError::new(
                ErrorKind::InvalidData,
                format!("{} '{}': not found, cannot update", S::LABEL, key),
            ))
        }
    }
}

impl<S,C> Display for LocalStore<S,C>
where
    S: Spec,
    C: Clone + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use flv_future_aio::task::run_block_on;
        let len = run_block_on(async { self.read().await.len() });
        write!(f, "{} Store count: {}", S::LABEL, len)
    }
}

impl<S,C> LocalStore<S,C>
where
    S: Spec + PartialEq,
    S::Status: PartialEq,
    C: Clone + Debug + PartialEq
{
    /// check store for entry, there are 3 possibilities (None,Same,Different)
    /// little bit efficient than cloning get
    pub async fn check(&self, value: &MetadataStoreObject<S,C>) -> CheckExist {
        if let Some(old_value) = self.read().await.get(value.key()) {
            if old_value == value {
                CheckExist::Same
            } else {
                CheckExist::Different
            }
        } else {
            CheckExist::None
        }
    }
}
