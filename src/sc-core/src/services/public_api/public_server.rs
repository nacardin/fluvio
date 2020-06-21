//!
//! # Public Sc Api Implementation
//!
//! Public service API allows 3rd party systems to invoke operations on Fluvio
//! Streaming Controller. Requests are received and dispatched to handlers
//! based on API keys.
//!

use std::sync::Arc;
use std::marker::PhantomData;

use async_trait::async_trait;
use futures::io::AsyncRead;
use futures::io::AsyncWrite;
use event_listener::Event;

use kf_service::api_loop;
use kf_service::call_service;
use kf_socket::InnerKfSocket;
use kf_socket::KfSocketError;
use kf_service::KfService;
use sc_api::ScPublicRequest;
use sc_api::ScPublicApiKey;
use k8_metadata_client::MetadataClient;
use flv_future_aio::zero_copy::ZeroCopyWrite;


use super::SharedPublicContext;

pub struct PublicService<C>(PhantomData<C>);

impl<C> PublicService<C> {
    pub fn new() -> Self {
        PublicService(PhantomData)
    }
}

#[async_trait]
impl<C, S> KfService<S> for PublicService<C>
where
    C: MetadataClient,
    S: AsyncWrite + AsyncRead + Unpin + Send + ZeroCopyWrite + 'static,
{
    type Context = SharedPublicContext<C>;
    type Request = ScPublicRequest;

    async fn respond(
        self: Arc<Self>,
        ctx: Self::Context,
        socket: InnerKfSocket<S>,
    ) -> Result<(), KfSocketError> {
        let (sink, mut stream) = socket.split();
        let mut api_stream = stream.api_stream::<ScPublicRequest, ScPublicApiKey>();
        let mut shared_sink = sink.as_shared();

        let end_event = Arc::new(Event::new());

        api_loop!(
            api_stream,


            ScPublicRequest::ApiVersionsRequest(request) => call_service!(
                request,
                super::api_version::handle_api_versions_request(request),
                shared_sink,
                "api version handler"
            ),

   
            ScPublicRequest::KfMetadataRequest(request) => call_service!(
                request,
                super::metadata::handle_kf_metadata_request(request, ctx.shared_context.clone()),
                shared_sink,
                "metadata request handler"
            ),

            ScPublicRequest::FlvCreateTopicRequest(request) => call_service!(
                request,
                super::topic::handle_create_topics_request(request, &ctx),
                shared_sink,
                "create topic handler"
            ),
            ScPublicRequest::FlvDeleteTopicRequest(request) => call_service!(
                request,
                super::topic::handle_delete_topic_request(request, &ctx),
                shared_sink,
                "delete topic handler"
            ),
            ScPublicRequest::FlvFetchTopicsRequest(request) => call_service!(
                request,
                super::topic::handle_fetch_topics_request(request, ctx.shared_context.clone()),
                shared_sink,
                "fetch topic handler"
            ),
            ScPublicRequest::FlvTopicCompositionRequest(request) => call_service!(
                request,
                super::metadata::handle_topic_composition_request(request, ctx.shared_context.clone()),
                shared_sink,
                "topic metadata handler"
            ),

            
            ScPublicRequest::FlvRegisterCustomSpuRequest(request) => call_service!(
                request,
                super::spu::handle_register_custom_spu_request(request, &ctx),
                shared_sink,
                "create custom spus handler"
            ),
            ScPublicRequest::FlvUnregisterCustomSpuRequest(request) => call_service!(
                request,
                super::spu::handle_unregister_custom_spu_request(request, &ctx),
                shared_sink,
                "delete custom spus handler"
            ),
            ScPublicRequest::FlvFetchSpusRequest(request) => call_service!(
                request,
                super::spu::handle_fetch_spus_request(request, ctx.shared_context.clone()),
                shared_sink,
                "fetch spus handler"
            ),

            ScPublicRequest::FlvCreateSpuGroupRequest(request) => call_service!(
                request,
                super::spg::handle_create_spu_group_request(request, &ctx),
                shared_sink,
                "create spu groups handler"
            ),
            ScPublicRequest::FlvDeleteSpuGroupRequest(request) => call_service!(
                request,
                super::spg::handle_delete_spu_group_request(request, &ctx),
                shared_sink,
                "delete spu groups handler"
            ),
            ScPublicRequest::FlvFetchSpuGroupsRequest(request) => call_service!(
                request,
                super::spg::handle_fetch_spu_groups_request(request, &ctx),
                shared_sink,
                "fetch spu groups handler"
            ),
            ScPublicRequest::UpdateMetadataRequest(request) => super::metadata::ClientMetadataController::handle_metadata_update(
                request,
                shared_sink.clone(), 
                end_event.clone(),
                ctx.shared_context.clone()),
            _ => {
                log::warn!("not actual protocol");
            }

        );

        // we are done with this tcp stream, notify any controllers use this strep
        end_event.notify(0);

        Ok(())
    }
}
