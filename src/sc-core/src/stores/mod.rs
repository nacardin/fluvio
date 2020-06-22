pub mod partition;
pub mod spu;
pub mod topic;
mod store;
mod kv_obj;
mod kv_context;

pub use store::*;
pub use kv_obj::*;
pub use kv_context::*;

pub use metadata::*;

mod metadata {

    use std::io::Error as IoError;

    use flv_metadata::k8::metadata::Spec as K8Spec;
    use flv_metadata::k8::metadata::K8Obj;

    use crate::stores::*;

    pub trait Spec: Default + Clone {
        const LABEL: &'static str;

        type Status: Status;
        type K8Spec: K8Spec;
        type Owner: Spec;

        type Key: Ord + Clone + ToString;

        // convert kubernetes objects into KV value
        fn convert_from_k8(k8_obj: K8Obj<Self::K8Spec>) -> Result<KVObject<Self>, IoError>;
    }

    pub trait Status: Default + Clone {}
}