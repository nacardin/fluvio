pub use flv_metadata::spg::*;

mod convert {

    use crate::objects::*;
    use super::*;

    impl From<SpuGroupSpec> for AllCreatableSpec {
        fn from(spec: SpuGroupSpec) -> Self {
            Self::SpuGroup(spec)
        }
    }

    impl DeleteSpec for SpuGroupSpec {

        fn into_request<K>(key: K) -> DeleteRequest where K: Into<Self::DeleteKey> {
            DeleteRequest::SpuGroup(key.into())
        }

    }

}
