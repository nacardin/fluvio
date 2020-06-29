mod concurrent_hashmap;
pub mod partition;
pub mod spu;
pub mod topic;
pub mod spg;
mod store;
mod kv_obj;
mod kv_context;
mod filter;

pub use store::*;
pub use kv_obj::*;
pub use kv_context::*;
pub use metadata::*;
pub use filter::*;
pub use concurrent_hashmap::*;


mod metadata {

    use std::io::Error as IoError;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::fmt::Display;
    use std::fmt::Debug;
    use std::io::ErrorKind;

    use flv_metadata::k8::metadata::K8Obj;
    use k8_obj_metadata::Spec as K8Spec;
    use flv_metadata::core::K8ExtendedSpec; 
    use flv_metadata::core::Spec;
    use crate::stores::*;

    pub trait StoreSpec: K8ExtendedSpec
       where <Self as Spec>::Owner: K8ExtendedSpec
    {
    
        // convert kubernetes objects into KV value
        fn convert_from_k8(k8_obj: K8Obj<Self::K8Spec>) -> Result<KVObject<Self>, IoError> 
            where Self::IndexKey: TryFrom<String> + Display,
                <Self::IndexKey as TryFrom<String>>::Error: Debug,
                <<Self as K8ExtendedSpec>::K8Spec as K8Spec>::Status: Into<Self::Status>,
                Self::K8Spec: Into<Self>,
        {
            let k8_name = k8_obj.metadata.name.clone();
            let result: Result<Self::IndexKey, _> = k8_name.try_into();
            match result {
                Ok(key) => {
                    // convert K8 Spec/Status into Metadata Spec/Status
                    let local_spec = k8_obj.spec.into();
                    let local_status = k8_obj.status.into();

                   
                    let ctx = KvContext::default().with_ctx(k8_obj.metadata);
                    let loca_kv = KVObject::new(key, local_spec, local_status).with_kv_ctx(ctx);

                    Ok(loca_kv)
                }
                Err(err) => Err(IoError::new(
                    ErrorKind::InvalidData,
                    format!("error converting key: {:#?}", err),
                )),
            }
        }
    }

    impl<T: ?Sized> StoreSpec for T 
        where T: K8ExtendedSpec,
            <T as Spec>::Owner: K8ExtendedSpec
    {}

}