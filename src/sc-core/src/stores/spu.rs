use std::ops::Deref;

use flv_metadata::spu::store::*;
use flv_metadata::store::*;
use flv_metadata::k8::metadata::ObjectMeta;
use internal_api::messages::SpuMsg;

pub type K8SpuMetadata = SpuMetadata<ObjectMeta>;
pub type K8SpuLocalStore = SpuLocalStore<ObjectMeta>;


pub struct SpuAdminStore(K8SpuLocalStore);

impl Deref for SpuAdminStore {
    type Target = K8SpuLocalStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SpuAdminStore {

    pub async fn all_spus_to_spu_msgs(&self) -> Vec<SpuMsg> {
        self.clone_specs()
            .await
            .into_iter()
            .map(|spu_spec| SpuMsg::update(spu_spec.into()))
            .collect()
    }


}