

use flv_metadata::spu::store::*;
use flv_metadata::spu::*;


use super::*;

pub type SpuAdminMd = SpuMetadata<K8MetaItem>;
pub type SpuAdminStore = SpuLocalStore<K8MetaItem>;



impl K8ExtendedSpec for SpuSpec {
    type K8Spec  = Self;
    type K8Status = Self::Status;
}

