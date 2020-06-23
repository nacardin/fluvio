
pub use flv_metadata::topic::*;

mod convert {

    use crate::objects::*;
    use super::*;

    
    impl From<TopicSpec> for AllCreatableSpec {
        fn from(spec: TopicSpec) -> Self {
            Self::Topic(spec)
        }
    }

    impl CreatableSpec for TopicSpec{}

}