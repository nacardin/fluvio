mod actions;
mod reducer;
mod controller;

pub use self::actions::*;
pub use self::controller::*;
pub use common::*;

mod common {

    use ::flv_metadata::topic::TopicSpec;
    use flv_metadata::store::actions::*;
    use crate::stores::K8MetaContext;



    use crate::metadata::K8ClusterStateDispatcher;

    pub type K8TopicChangeDispatcher<C> = K8ClusterStateDispatcher<TopicSpec, C>;
    pub type TopicWSAction = WSAction<TopicSpec,K8MetaContext>;
    pub type TopicLSChange = LSChange<TopicSpec,K8MetaContext>;
}
