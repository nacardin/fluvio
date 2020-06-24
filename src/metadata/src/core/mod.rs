use std::fmt::Debug;

use kf_protocol::Encoder;
use kf_protocol::Decoder;

/// metadata driver
pub trait MetadataStoreDriver {
    type Metadata;
}

pub trait Spec: Encoder + Decoder + Debug {
    const LABEL: &'static str;
    type Status: Status;
    type Owner: Spec;
    type IndexKey: Ord + Clone + ToString;
    
}

pub trait Status: Encoder + Decoder + Debug   {

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