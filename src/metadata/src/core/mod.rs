pub use core_model::*;
pub use context::*;

mod context {

    pub type DefaultMetadataContext = MetadataContext<String>;

    pub trait MetadataItem: Clone + Default {}

    #[derive(Default)]
    pub struct MetadataContext<C> {
        item: C,
        owner: Option<C>
    }

    impl <C>From<C> for MetadataContext<C> {
        fn from(item: C) -> Self {
            Self {
                item,
                owner: None
            }
        }
    }

    impl<C> MetadataContext<C> {
     
        pub fn item(&self) -> &C {
            &self.item
        }

        pub fn owner(&self) -> Option<&C> {
            self.owner.as_ref()
        }
        pub fn set_owner(&mut self, ctx: C) {
            self.owner = Some(ctx);
        }
    }

    impl <C> MetadataContext<C>
        where C: MetadataItem
    {

        pub fn create_child(&self,child_context: C) -> Self {
            Self {
                item: child_context,
                owner: Some(self.item.clone())
            }
        }
    }



}

mod core_model {

    use std::fmt::Debug;

    use kf_protocol::Encoder;
    use kf_protocol::Decoder;

    /// metadata driver
    pub trait MetadataStoreDriver {
        type Metadata;
    }



    pub trait Spec: Encoder + Decoder + Debug + Clone {
        const LABEL: &'static str;
        type Status: Status;
        type Owner: Spec;
        type IndexKey: Debug + Ord + Clone + ToString;
        
    }

    pub trait Status: Encoder + Decoder + Debug + Clone  {

    }

    /// for deleting objects
    pub trait Removable {

        type DeleteKey: Encoder + Decoder;

        
    }

    /// marker trait for creating
    pub trait Creatable {}

    /// Represents some metadata object
    pub struct MetadataObj<S,P> where P: MetadataStoreDriver, S:Spec {
        pub name: String,
        pub metadata: P::Metadata,
        pub spec: S,
        pub status: S::Status
    }

}



#[cfg(feature = "k8")]
pub use k8::*;

#[cfg(feature = "k8")]
mod k8 {

    use k8_obj_metadata::Spec as K8Spec;
    use k8_obj_metadata::Status as K8Status;
    use crate::k8::metadata::ObjectMeta;
    use crate::k8::metadata::K8Obj;
    use super::*;

    pub trait K8ExtendedSpec: Spec
    {
        type K8Spec: K8Spec;
        type K8Status: K8Status;

        fn convert_from_k8(
            k8_obj: K8Obj<Self::K8Spec>,
        ) -> Result<MetadataStoreObject<Self,ObjectMeta>, IoError>
        where
            Self::IndexKey: TryFrom<String> + Display,
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

                    let ctx: MetadataContext<Self,ObjectMeta> = k8_obj.metadata.into();
                    let loca_kv =
                        MetadataStoreObject::new(key, local_spec, local_status).with_context(ctx);

                    Ok(loca_kv)
                }
                Err(err) => Err(IoError::new(
                    ErrorKind::InvalidData,
                    format!("error converting key: {:#?}", err),
                )),
            }
        }
    }

    pub type K8MetadataContext = MetadataContext<ObjectMeta>;


}
