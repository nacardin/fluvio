//!
//! # Update KV Store with SPU status (online/offline)
//!
use std::fmt::Display;
use std::convert::Into;

use log::trace;
use log::warn;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use async_trait::async_trait;

use flv_metadata::topic::TopicSpec;
use flv_metadata::partition::PartitionSpec;
use flv_metadata::spu::SpuSpec;
use flv_metadata::k8::metadata::InputK8Obj;
use flv_metadata::core::Spec;
use flv_metadata::store::actions::*;
use flv_metadata::store::*;
use flv_types::log_on_err;
use flv_metadata::k8::metadata::Spec as K8Spec;
use flv_metadata::k8::metadata::UpdateK8ObjStatus;
use k8_metadata_client::MetadataClient;
use k8_metadata_client::SharedClient;

use crate::ScServerError;
use crate::stores::*;
use crate::core::WSUpdateService;


pub struct K8WSUpdateService<C>(SharedClient<C>);

impl<C> Clone for K8WSUpdateService<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C> K8WSUpdateService<C>
where
    C: MetadataClient,
{
    pub fn new(client: SharedClient<C>) -> Self {
        Self(client)
    }

    pub fn client(&self) -> &C {
        &self.0
    }

    pub fn own_client(&self) -> SharedClient<C> {
        self.0.clone()
    }

    pub async fn add<S>(&self, value: MetadataStoreObject<S,K8MetaItem>) -> Result<(), C::MetadataClientError>
    where
        S: K8ExtendedSpec + Into<<S as K8ExtendedSpec>::K8Spec>,
        <S as Spec>::Owner: K8ExtendedSpec,
        S::Status: PartialEq,
        S::IndexKey: Display,
        <S as K8ExtendedSpec>::K8Spec: DeserializeOwned + Serialize + Send,
        <<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status: DeserializeOwned + Serialize + Send,
    {
        debug!("Adding: {}:{}", S::LABEL, value.key());
        trace!("adding KV {:#?} to k8 kv", value);

        let (key, spec, _status,ctx) = value.parts();
        let k8_spec: S::K8Spec = spec.into();
        
        if let Some(parent_metadata) = ctx.owner() {
            let item_name = key.to_string();

            let new_k8 = InputK8Obj::new(
                k8_spec,
                parent_metadata
                    .make_child_input_metadata::<<<S as Spec>::Owner as K8ExtendedSpec>::K8Spec>(
                        item_name,
                    ),
            );

            self.0.apply(new_k8).await.map(|_| ())
        } else {
            let new_k8 = InputK8Obj::new(k8_spec, ctx.item_owned().into());

            self.0
                .apply(new_k8)
                .await
                .map(|_| ())
                .map_err(|err| err.into())
        }
    }

    /// only update the status
    async fn update_status<S>(
        &self,
        value: MetadataStoreObject<S,K8MetaItem>,
    ) -> Result<(), C::MetadataClientError>
    where
        S: K8ExtendedSpec,
        S::IndexKey: Display,
        <S as Spec>::Owner: K8ExtendedSpec,
        S::Status: Display + Into<<<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status>,
        <S as K8ExtendedSpec>::K8Spec: Serialize + DeserializeOwned + Send + Sync,
        <<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status:
            Serialize + DeserializeOwned + Send + Sync,
    {
        debug!(
            "K8 Update Status: {} key: {} value: {}",
            S::LABEL,
            value.key(),
            value.status
        );
        trace!("status update: {:#?}", value.status);

        
        let (_key,_spec,status,ctx) = value.parts();
        let k8_status: <<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status = status.into();

        
        let k8_input: UpdateK8ObjStatus<S::K8Spec> = UpdateK8ObjStatus {
            api_version: S::K8Spec::api_version(),
            kind: S::K8Spec::kind(),
            metadata: ctx.item_owned().into(),
            status: k8_status,
            ..Default::default()
        };

            self.0.update_status(&k8_input).await.map(|_| ())
        
    }

    /// update both spec and status
    pub async fn update_spec<S>(
        &self,
        value: MetadataStoreObject<S,K8MetaItem>,
    ) -> Result<(), C::MetadataClientError>
    where
        S: K8ExtendedSpec + Into<<S as K8ExtendedSpec>::K8Spec>,
        <S as Spec>::Owner: K8ExtendedSpec,
        S::IndexKey: Display,
        S::Status: Into<<<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status>,
        <S as K8ExtendedSpec>::K8Spec: Serialize + DeserializeOwned + Send,
        <<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status: Serialize + DeserializeOwned + Send,
    {
        debug!("K8 Update Spec: {} key: {}", S::LABEL, value.key());
        trace!("K8 Update Spec: {:#?}", value);

        let (_key,spec,_status,ctx) = value.parts();

        let k8_spec: <S as K8ExtendedSpec>::K8Spec = spec.into();
        
        trace!("updating spec: {:#?}", k8_spec);

        let k8_input: InputK8Obj<S::K8Spec> = InputK8Obj {
            api_version: S::K8Spec::api_version(),
            kind: S::K8Spec::kind(),
            metadata: ctx.item_owned().into(),
            spec: k8_spec,
            ..Default::default()
        };

        self.0.apply(k8_input).await.map(|_| ())
       
    }

    async fn inner_process<S>(&self, action: WSAction<S,K8MetaItem>) -> Result<(), ScServerError>
    where
        S: K8ExtendedSpec + Into<<S as K8ExtendedSpec>::K8Spec>,
        S::IndexKey: Display,
        <S as Spec>::Owner: K8ExtendedSpec,
        S::Status: PartialEq + Display,
        <S as K8ExtendedSpec>::K8Spec: Serialize + DeserializeOwned + Send + Sync,
        <<S as K8ExtendedSpec>::K8Spec as K8Spec>::Status:
            From<S::Status> + Serialize + DeserializeOwned + Send + Sync,
    {
        match action {
            WSAction::Add(value) => log_on_err!(self.add(value).await),
            WSAction::UpdateStatus(value) => log_on_err!(self.update_status(value).await),
            WSAction::UpdateSpec(value) => log_on_err!(self.update_spec(value).await),
            WSAction::Delete(_key) => warn!("delete not yet implemented"),
        }
        Ok(())
    }
}

#[async_trait]
impl<C> WSUpdateService for K8WSUpdateService<C>
where
    C: MetadataClient,
{
    async fn update_spu(&self, ws_actions: WSAction<SpuSpec,K8MetaItem>) -> Result<(), ScServerError> {
        let service = self.clone();
        service.inner_process(ws_actions).await?;
        Ok(())
    }

    async fn update_topic(&self, ws_actions: WSAction<TopicSpec,K8MetaItem>) -> Result<(), ScServerError> {
        let service = self.clone();
        service.inner_process(ws_actions).await?;
        Ok(())
    }

    async fn update_partition(
        &self,
        ws_actions: WSAction<PartitionSpec,K8MetaItem>,
    ) -> Result<(), ScServerError> {
        let service = self.clone();
        service.inner_process(ws_actions).await?;
        Ok(())
    }
}
