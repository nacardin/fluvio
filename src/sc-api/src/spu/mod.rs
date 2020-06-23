pub use flv_metadata::spu::*;

mod convert {
    
    use crate::objects::*;
    use super::*;
    
    impl From<CustomSpuSpec> for AllCreatableSpec {
        fn from(spec: CustomSpuSpec) -> Self {
            Self::CustomSpu(spec)
        }
    }

    impl CreatableSpec for CustomSpuSpec{}
}