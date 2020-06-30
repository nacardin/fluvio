//!
//! # Key/Value Context
//!
//! Key/Value Contexts are required by KV store for modifications and owner_references.
//! Controller treats these objects as opaque cookies which are converted to Metadata by
//! the KV client.
use flv_metadata::k8::metadata::ObjectMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct MetadataContext {
    pub item_ctx: Option<ObjectMeta>,
    pub parent_ctx: Option<ObjectMeta>,
}

impl MetadataContext {
    pub fn with_ctx(mut self, ctx: ObjectMeta) -> Self {
        self.item_ctx = Some(ctx);
        self
    }

    pub fn with_parent_ctx(mut self, ctx: ObjectMeta) -> Self {
        self.parent_ctx = Some(ctx);
        self
    }

    pub fn make_parent_ctx(&self) -> MetadataContext {
        if self.item_ctx.is_some() {
            MetadataContext::default().with_parent_ctx(self.item_ctx.as_ref().unwrap().clone())
        } else {
            MetadataContext::default()
        }
    }
}

impl ::std::default::Default for MetadataContext {
    fn default() -> Self {
        MetadataContext {
            item_ctx: None,
            parent_ctx: None,
        }
    }
}
