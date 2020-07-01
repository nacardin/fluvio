use std::fmt::Debug;

use crate::core::*;

// -----------------------------------
// Data Structures
// -----------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataStoreObject<S,C>
where
    S: Spec,
    C: MetadataItem
{
    pub spec: S,
    pub status: S::Status,
    pub key: S::IndexKey,
    pub ctx: MetadataContext<C>
}

impl<S,C> MetadataStoreObject<S,C>
where
    S: Spec,
    C: MetadataItem
{
    pub fn new<J>(key: J, spec: S, status: S::Status) -> Self
    where
        J: Into<S::IndexKey>,
        C: Default
    {
        Self {
            key: key.into(),
            spec,
            status,
            ctx: MetadataContext::default(),
        }
    }

    pub fn new_with_context<J>(key: J, spec: S, ctx:MetadataContext<C>) -> Self
    where
        J: Into<S::IndexKey>,
    {
        Self {
            key: key.into(),
            spec,
            status: S::Status::default(),
            ctx,
        }
    }

    pub fn with_spec<J>(key: J, spec: S) -> Self
    where
        J: Into<S::IndexKey>, C: Default
    {
        Self::new(key.into(), spec, S::Status::default())
    }

    pub fn with_context(mut self, ctx: MetadataContext<C>) -> Self {
        self.ctx = ctx;
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

    pub fn ctx(&self) -> &MetadataContext<C> {
        &self.ctx
    }

    pub fn ctx_owned(&self) -> MetadataContext<C> {
        self.ctx.clone()
    }

    pub fn set_ctx(&mut self, ctx: MetadataContext<C>) {
        self.ctx = ctx;
    }

    pub fn parts(self) -> (S::IndexKey, S, MetadataContext<C>) {
        (self.key, self.spec, self.ctx)
    }

    pub fn is_owned<U>(&self, uid: U) -> bool 
        where U: AsRef<C>,
            C: PartialEq
    {

        match self.ctx().owner() {
            Some(parent) => parent == uid.as_ref(),
            None => false,
        }
    }
}


impl<S,C> Into<(S::IndexKey, S, S::Status)> for MetadataStoreObject<S,C>
where
    S: Spec,
    C: MetadataItem
{
    fn into(self) -> (S::IndexKey, S, S::Status) {
        (self.key, self.spec, self.status)
    }
}
