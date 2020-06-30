mod actions;
mod reducer;
mod controller;

pub use self::actions::*;
pub use self::controller::*;
pub use common::*;

mod common {

    use ::flv_metadata::topic::TopicSpec;
    use flv_metadata::k8::metadata::ObjectMeta;
    use crate::stores::actions::*;


    use crate::metadata::K8ClusterStateDispatcher;

    pub type K8TopicChangeDispatcher<C> = K8ClusterStateDispatcher<TopicSpec, C>;
    pub type TopicWSAction = WSAction<TopicSpec,ObjectMeta>;
    pub type TopicLSChange = LSChange<TopicSpec,ObjectMeta>;
}
