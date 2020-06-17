mod public_server;

mod flv;
mod kf;

mod api {
   pub use super::flv::*;
   pub use super::kf::*;
}




pub use context::*;

mod context {

    use std::sync::Arc;
    use std::fmt::Debug;

    use log::info;
    use log::debug;
    use serde::Serialize;
    use serde::de::DeserializeOwned;


    use kf_service::KfApiServer;
    
    use sc_api::*;
    use k8_metadata_client::MetadataClient;
    use k8_metadata::metadata::InputObjectMeta;
    use k8_metadata::metadata::InputK8Obj;
    use k8_metadata::metadata::K8List;
    use k8_metadata::metadata::Spec as K8Spec;

    use crate::core::SharedContext;
    use crate::metadata::K8WSUpdateService;
    use crate::core::Context;
    use super::public_server::PublicService;

    pub type SharedPublicContext<C> = Arc<PublicContext<C>>;

    pub type PublicApiServer<C> =
    KfApiServer<ScPublicRequest, ScPublicApiKey, SharedPublicContext<C>, PublicService<C>>;

    /// create public server
    pub fn create_public_server<C>(
        metadata: SharedContext,
        k8_ws: K8WSUpdateService<C>,
        namespace: String,
    ) -> PublicApiServer<C>
    where
        C: MetadataClient,
    {
        let addr = metadata.config().public_endpoint.clone();
        info!("start public api service at: {}", addr);

        KfApiServer::new(
            addr,
            Arc::new(PublicContext {
                metadata,
                k8_ws,
                namespace,
            }),
            PublicService::new(),
        )
    }

    #[derive(Clone)]
    pub struct PublicContext<C> {
        pub metadata: SharedContext,
        pub k8_ws: K8WSUpdateService<C>,
        pub namespace: String,
    }

    impl<C> PublicContext<C>
    where
        C: MetadataClient,
    {
        pub fn k8_client(&self) -> &C {
            self.k8_ws.client()
        }

        pub fn k8_ws(&self) -> &K8WSUpdateService<C> {
            &self.k8_ws
        }

        pub fn metadata(&self) -> &Context {
            &self.metadata
        }

        /// Create input metadata for our context
        /// which has namespace
        pub async fn create<S>(&self, name: String, spec: S) -> Result<(), C::MetadataClientError>
        where
            S: K8Spec + Serialize + Default + Debug + Clone + DeserializeOwned + Send,
            <S as K8Spec>::Status: Default + Debug + Serialize + DeserializeOwned + Send,
        {
            debug!("creating k8 spec: {:#?}", spec);
            let input = InputK8Obj {
                api_version: S::api_version(),
                kind: S::kind(),
                metadata: InputObjectMeta {
                    name,
                    namespace: self.namespace.clone(),
                    ..Default::default()
                },
                spec,
                ..Default::default()
            };

            let client = self.k8_ws.client();
            client.apply(input).await?;

            Ok(())
        }

        /// Create input metadata for our context
        /// which has namespace
        pub async fn delete<S>(&self, name: &str) -> Result<(), C::MetadataClientError>
        where
            S: K8Spec + Serialize + Default + Debug + Clone + DeserializeOwned,
            <S as K8Spec>::Status: Default + Debug + DeserializeOwned,
        {
            debug!("deleting k8 obj: {}", name);
            let meta = InputObjectMeta {
                name: name.to_owned(),
                namespace: self.namespace.clone(),
                ..Default::default()
            };

            let client = self.k8_ws.client();
            client.delete_item::<S, _>(&meta).await?;

            Ok(())
        }

        /// retrieve all items in the namespace
        pub async fn retrieve_items<S>(&self) -> Result<K8List<S>, C::MetadataClientError>
        where
            S: K8Spec,
            K8List<S>: DeserializeOwned,
        {
            let client = self.k8_ws.client();
            client.retrieve_items::<S, _>(self.namespace.clone()).await
        }
    }
}