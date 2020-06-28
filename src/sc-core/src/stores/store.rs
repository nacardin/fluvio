use std::sync::Arc;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::collections::BTreeMap;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;


use super::StoreSpec;
use flv_util::SimpleConcurrentBTreeMap;
use flv_metadata::core::K8ExtendedSpec;  
use flv_metadata::core::Spec;

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

impl<S> ::std::cmp::PartialEq for LocalStore<S>
where
    S: StoreSpec + PartialEq,
    <S as Spec>::Owner: K8ExtendedSpec,
    S::Status: PartialEq ,
{
    fn eq(&self, other: &LocalStore<S>) -> bool {
        for (key, val) in self.0.read().iter() {
            let other_list = other.0.read();
            let other_val = match other_list.get(key) {
                Some(val) => val,
                None => return false,
            };
            if val != other_val {
                return false;
            }
        }
        true
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
    pub fn read(&self) -> RwLockReadGuard<BTreeMap<S::IndexKey,KVObject<S>>>
    {
        self.0.read()
    }

    #[inline(always)]
    pub fn write(&self) -> RwLockWriteGuard<BTreeMap<S::IndexKey,KVObject<S>>>
    {
        self.0.write()
    } 
    
    pub fn insert(&self, value: KVObject<S>) -> Option<KVObject<S>> {
        self.write().insert(value.key_owned(), value)
    }


    pub fn for_each<F>(&self, func: F)
    where
        F: FnMut(&'_ KVObject<S>),
    {
        self.read().values().for_each(func);
    }


    /// get copy of the value ref by key
    pub fn value<K: ?Sized>(&self, key: &K) -> Option<KVObject<S>>
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        match self.read().get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }


    /// get copy of the spec ref by key
    pub fn spec<K: ?Sized>(&self, key: &K) -> Option<S>
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        match self.read().get(key) {
            Some(value) => Some(value.spec.clone()),
            None => None,
        }
    }

    pub fn find_and_do<K, F>(&self, key: &K, mut func: F) -> Option<()>
    where
        F: FnMut(&'_ KVObject<S>),
        K: Ord,
        S::IndexKey: Borrow<K>,
    {
        if let Some(value) = self.read().get(key) {
            func(value);
            Some(())
        } else {
            None
        }
    }

    
    pub fn contains_key<K: ?Sized>(&self, key: &K) -> bool
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        self.read().contains_key(key)
    }

    pub fn remove<K: ?Sized>(&self, key: &K) -> Option<KVObject<S>>
    where
        S::IndexKey: Borrow<K>,
        K: Ord,
    {
        self.write().remove(key)
    }

    pub fn count(&self) -> i32 {
        self.read().len() as i32
    }

    /// get copy of keys
    pub fn clone_keys(&self) -> Vec<S::IndexKey> {
        self.read().keys().cloned().collect()
    }

    pub fn clone_values(&self) -> Vec<KVObject<S>> {
        self.read().values().cloned().collect()
    }

    pub fn clone_specs(&self) -> Vec<S> {
        self.read()
            .values()
            .map(|kv| kv.spec.clone())
            .collect()
    }

    

    /// update status
    pub fn update_status<K: ?Sized>(&self, key: &K, status: S::Status) -> Result<(), IoError>
    where
        S::IndexKey: Borrow<K>,
        K: Display + Ord,
    {
        if let Some(old_kv) = self.write().get_mut(key) {
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
        write!(
            f,
            "{} Store count: {}",
            S::LABEL,
            self.read().len()
        )
    }
}
