
pub use flv_metadata::topic::*;

mod convert {

    use super::*;
    
    use crate::objects::CreatableSpec;
    

    impl CreatableSpec for TopicSpec{}

}