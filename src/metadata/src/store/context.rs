//!
//! # Key/Value Context
//!
//! Key/Value Contexts are required by KV store for modifications and owner_references.
//! Controller treats these objects as opaque cookies which are converted to Metadata by
//! the KV client.




mod k8 {

    use std::ops::Deref;

    use crate::k8::metadata::ObjectMeta;
    use crate::core::*;

    pub type K8MetadataContext = MetadataContext<ObjectMeta>;
    
    impl Deref for K8MetadataContext {
        type Target = ObjectMeta;

        fn deref(&self) -> &Self::Target {
            &self.item()
        }
    }

}