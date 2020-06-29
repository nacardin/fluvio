use std::sync::Arc;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::collections::BTreeMap;


use flv_future_aio::sync::RwLockReadGuard;
use flv_future_aio::sync::RwLockWriteGuard;



use super::SimpleConcurrentBTreeMap;
use flv_metadata::core::K8ExtendedSpec;  
use flv_metadata::core::Spec;
use super::StoreSpec;

use super::*;

/// Local state in memory
#[derive(Debug)]
pub struct LocalStore<S>(SimpleConcurrentBTreeMap<S::IndexKey, KVObject<S>>)
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec;

impl<S> Default for LocalStore<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
{
    fn default() -> Self {
        LocalStore(SimpleConcurrentBTreeMap::new())
    }
}

impl<S> LocalStore<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
{
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// read access
    #[inline(always)]
    pub async fn read<'a>(&'a self) -> RwLockReadGuard<'a,BTreeMap<S::IndexKey,KVObject<S>>>
    {
        self.0.read().await
    }

    #[inline(always)]
    pub async fn write<'a>(&'a self) -> RwLockWriteGuard<'a,BTreeMap<S::IndexKey,KVObject<S>>>
    {
        self.0.write().await
    } 
    
    pub async fn insert(&self, value: KVObject<S>) -> Option<KVObject<S>> {
        self.write().await.insert(value.key_owned(), value)
    }


    pub async fn for_each<F>(&self, func: F)
    where
        F: FnMut(&'_ KVObject<S>),
    {
        self.read().await.values().for_each(func);
    }


    /// get copy of the value ref by key
    pub async fn value<K: ?Sized>(&self, key: &K) -> Option<KVObject<S>>
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
        F: FnMut(&'_ KVObject<S>),
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

    pub async fn remove<K: ?Sized>(&self, key: &K) -> Option<KVObject<S>>
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

    pub async fn clone_values(&self) -> Vec<KVObject<S>> {
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

impl<S> Display for LocalStore<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        use flv_future_aio::task::run_block_on;
        let len = run_block_on(async { self.read().await.len()});
        write!(
            f,
            "{} Store count: {}",
            S::LABEL,
            len
        )
    }
}
