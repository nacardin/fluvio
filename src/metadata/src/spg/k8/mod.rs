//!
//! # Cluster
//!
//! Interface to the Cluster metadata in K8 key value store
//!
mod spec;
mod status;

pub use self::spec::*;
pub use self::status::*;

mod ext {
    use crate::core::K8ExtendedSpec;
    use crate::spg::SpuGroupSpec;

    use super::K8SpuGroupSpec;

    impl K8ExtendedSpec for SpuGroupSpec {
        type K8Spec = K8SpuGroupSpec;
    }
}
