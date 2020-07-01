mod actions;
mod reducer;
mod controller;

pub use self::actions::*;
pub use self::controller::*;
pub use common::*;

mod common {

    use ::flv_metadata::spu::SpuSpec;
    use flv_metadata::store::actions::*;
    use crate::stores::K8MetaContext;


    use crate::metadata::K8ClusterStateDispatcher;

    pub type K8SpuChangeDispatcher<C> = K8ClusterStateDispatcher<SpuSpec, C>;

    pub type SpuLSChange = LSChange<SpuSpec,K8MetaContext>;
}
