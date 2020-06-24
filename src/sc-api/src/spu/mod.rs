pub use flv_metadata::spu::*;

mod convert {
    
    use crate::objects::*;
    use super::*;
    
    impl From<CustomSpuSpec> for AllCreatableSpec {
        fn from(spec: CustomSpuSpec) -> Self {
            Self::CustomSpu(spec)
        }
    }

    impl DeleteSpec for CustomSpuSpec  {

        fn into_request<K>(key: K) -> DeleteRequest where K: Into<Self::DeleteKey> {
            DeleteRequest::CustomSpu(key.into())
        }

    }

}