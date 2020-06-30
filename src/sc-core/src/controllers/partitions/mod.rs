mod actions;
mod controller;
mod reducer;

pub use self::actions::*;
pub use self::controller::*;
pub use common::*;

mod common {

    use ::flv_metadata::partition::PartitionSpec;
    use flv_metadata::k8::metadata::ObjectMeta;

    use crate::stores::actions::WSAction;
    use crate::stores::actions::LSChange;
    use crate::metadata::K8ClusterStateDispatcher;
    

    pub type K8PartitionChangeDispatcher<C> = K8ClusterStateDispatcher<PartitionSpec, C>;
    pub type PartitionWSAction = WSAction<PartitionSpec,ObjectMeta>;
    pub type PartitionLSChange = LSChange<PartitionSpec,ObjectMeta>;

}
