//!
//! # Cluster
//!
//! Interface to the Cluster metadata in K8 key value store
//!
mod spec;


pub use self::spec::*;


mod ext {
    use k8_obj_metadata::Status as K8Status;
    
    use crate::core::K8ExtendedSpec;
    use crate::spg::SpuGroupSpec;
    use crate::spg::SpuGroupStatus;

    use super::K8SpuGroupSpec;

    /// implement k8 status for spu group status because they are same
    impl K8Status for SpuGroupStatus{}

    /// for group status, we have custom spu group spec
    impl K8ExtendedSpec for SpuGroupSpec {
        type K8Spec  = K8SpuGroupSpec;
        type K8Status = Self::Status;
    }

}
