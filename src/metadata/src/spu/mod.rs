mod spec;
mod status;

pub use self::spec::*;
pub use self::status::*;

#[cfg(feature = "k8")]
mod k8;
#[cfg(feature = "k8")]
pub use k8::*;


mod metadata {

    use crate::core::*;
    use super::*;

    impl Spec for SpuSpec {
        const LABEL: &'static str = "SPU";
        type Key = String;
        type Status = SpuStatus;
        type Owner = SpuSpec;
    }

    impl Status for SpuStatus {}
}