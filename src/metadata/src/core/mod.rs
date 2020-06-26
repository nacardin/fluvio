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


#[cfg(feature = "k8")]
pub use k8::*;

#[cfg(feature = "k8")]
mod k8 {

    use k8_obj_metadata::Spec as K8Spec;
    use k8_obj_metadata::Status as K8Status;
    use super::*;

    pub trait K8ExtendedSpec: Spec
    {
        type K8Spec: K8Spec;
        type K8Status: K8Status;
    }


}
