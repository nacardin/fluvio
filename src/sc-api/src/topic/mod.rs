
pub use flv_metadata::topic::*;

mod convert {

    use crate::objects::*;
    use super::*;

    
    impl From<TopicSpec> for AllCreatableSpec {
        fn from(spec: TopicSpec) -> Self {
            Self::Topic(spec)
        }
    }


    impl DeleteSpec for TopicSpec  {

        fn into_request<K>(key: K) -> DeleteRequest where K: Into<Self::DeleteKey> {
            DeleteRequest::Topic(key.into())
        }

    }

}