
use flv_metadata::spg::*;
use flv_metadata::spg::store::*;

use super::*;

/// for group status, we have custom spu group spec
impl K8ExtendedSpec for SpuGroupSpec {
    type K8Spec  = K8SpuGroupSpec;
    type K8Status = Self::Status;
}

pub type  SpgAdminMd = SpuGroupMetadata<K8MetaContext>;
pub type  SpgAdminStore = SpuGroupLocalStore<K8MetaContext>;