pub use flv_metadata::spg::*;

mod convert {

    use crate::objects::*;
    use super::*;

    impl From<SpuGroupSpec> for AllCreatableSpec {
        fn from(spec: SpuGroupSpec) -> Self {
            Self::SpuGroup(spec)
        }
    }

    impl CreatableSpec for SpuGroupSpec {}
}
