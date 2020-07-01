

use flv_metadata::spu::store::*;
use flv_metadata::spu::*;
use flv_metadata::k8::metadata::ObjectMeta;


use super::*;

pub type SpuAdminMd = SpuMetadata<K8MetaContext>;
pub type SpuAdminStore = SpuLocalStore<ObjectMeta>;



impl K8ExtendedSpec for SpuSpec {
    type K8Spec  = Self;
    type K8Status = Self::Status;
}

