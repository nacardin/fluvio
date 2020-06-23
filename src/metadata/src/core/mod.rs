
use kf_protocol::Encoder;
use kf_protocol::Decoder;

/// metadata driver
pub trait MetadataStoreDriver {
    type Metadata;
}

pub trait Spec: Encoder + Decoder + Default + Clone {
    const LABEL: &'static str;
    type Status: Status;
    type Owner: Spec;
    type Key: Ord + Clone + ToString;
}

pub trait Status: Encoder + Decoder  + Default + Clone  {

}

/// Represents some metadata object
pub struct MetadataObj<S,P> where P: MetadataStoreDriver, S:Spec {
    pub name: String,
    pub metadata: P::Metadata,
    pub spec: S,
    pub status: S::Status
}