mod actions;
mod reducer;
mod controller;

pub use self::actions::*;
pub use self::reducer::*;
pub use self::controller::*;
pub use common::*;

mod common {

    use ::flv_metadata::topic::TopicSpec;
    use crate::core::common::LSChange;
    use crate::core::common::WSAction;

    use crate::metadata::K8ClusterStateDispatcher;

    pub type K8TopicChangeDispatcher<C> = K8ClusterStateDispatcher<TopicSpec, C>;
    pub type TopicWSAction = WSAction<TopicSpec>;
    pub type TopicLSChange = LSChange<TopicSpec>;
}
