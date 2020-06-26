use std::fmt;
use std::fmt::Display;

use flv_metadata::core::K8ExtendedSpec;
use flv_metadata::core::Spec;

use super::*;

// -----------------------------------
// Data Structures
// -----------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct KVObject<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
{
    pub spec: S,
    pub status: S::Status,
    pub key: S::IndexKey,
    pub kv_ctx: KvContext,
}

impl<S> KVObject<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
{
    pub fn new<J>(key: J, spec: S, status: S::Status) -> Self
    where
        J: Into<S::IndexKey>,
    {
        Self {
            key: key.into(),
            spec,
            status,
            kv_ctx: KvContext::default(),
        }
    }

    pub fn new_with_context<J>(key: J, spec: S, kv_ctx: KvContext) -> Self
    where
        J: Into<S::IndexKey>,
    {
        Self {
            key: key.into(),
            spec,
            status: S::Status::default(),
            kv_ctx,
        }
    }

    pub fn with_spec<J>(key: J, spec: S) -> Self
    where
        J: Into<S::IndexKey>,
    {
        Self::new(key.into(), spec, S::Status::default())
    }

    pub fn with_kv_ctx(mut self, kv_ctx: KvContext) -> Self {
        self.kv_ctx = kv_ctx;
        self
    }

    pub fn key(&self) -> &S::IndexKey {
        &self.key
    }

    pub fn key_owned(&self) -> S::IndexKey {
        self.key.clone()
    }

    pub fn my_key(self) -> S::IndexKey {
        self.key
    }

    pub fn spec(&self) -> &S {
        &self.spec
    }
    pub fn status(&self) -> &S::Status {
        &self.status
    }

    pub fn kv_ctx(&self) -> &KvContext {
        &self.kv_ctx
    }

    pub fn set_ctx(&mut self, new_ctx: &KvContext) {
        self.kv_ctx = new_ctx.clone();
    }

    pub fn parts(self) -> (S::IndexKey, S, KvContext) {
        (self.key, self.spec, self.kv_ctx)
    }

    pub fn is_owned(&self, uid: &str) -> bool {
        match &self.kv_ctx.parent_ctx {
            Some(parent) => parent.uid == uid,
            None => false,
        }
    }
}

impl<S> fmt::Display for KVObject<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
    S::IndexKey: Display,
{
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KV {} key: {}", S::LABEL, self.key())
    }
}

impl<S> Into<(S::IndexKey, S, S::Status)> for KVObject<S>
where
    S: StoreSpec,
    <S as Spec>::Owner: K8ExtendedSpec,
{
    fn into(self) -> (S::IndexKey, S, S::Status) {
        (self.key, self.spec, self.status)
    }
}
