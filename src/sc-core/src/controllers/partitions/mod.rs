mod actions;
mod controller;
mod reducer;

pub use self::actions::*;
pub use self::controller::*;
pub use common::*;

mod common {

    use ::flv_metadata::partition::PartitionSpec;
    use crate::core::common::WSAction;
    use crate::core::common::LSChange;
    use crate::metadata::K8ClusterStateDispatcher;

    pub type K8PartitionChangeDispatcher<C> = K8ClusterStateDispatcher<PartitionSpec, C>;
    pub type PartitionWSAction = WSAction<PartitionSpec>;

    pub type PartitionLSChange = LSChange<PartitionSpec>;

}
