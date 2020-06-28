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
use sc_api::AdminPublicApiKey;
use sc_api::AdminPublicRequest;
use k8_metadata_client::MetadataClient;
use flv_future_aio::zero_copy::ZeroCopyWrite;

use super::*;

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
    type Request = AdminPublicRequest;

    async fn respond(
        self: Arc<Self>,
        ctx: Self::Context,
        socket: InnerKfSocket<S>,
    ) -> Result<(), KfSocketError> {
        let (sink, mut stream) = socket.split();
        let mut api_stream = stream.api_stream::<AdminPublicRequest, AdminPublicApiKey>();
        let mut shared_sink = sink.as_shared();

        let end_event = Arc::new(Event::new());

        api_loop!(
            api_stream,

            AdminPublicRequest::ApiVersionsRequest(request) => call_service!(
                request,
                super::api_version::handle_api_versions_request(request),
                shared_sink,
                "api version handler"
            ),

            AdminPublicRequest::CreateRequest(request) => call_service!(
                request,
                super::create::handle_create_request(request, &ctx),
                shared_sink,
                "create  handler"
            ),
            AdminPublicRequest::DeleteRequest(request) => call_service!(
                request,
                super::delete::handle_delete_request(request, &ctx),
                shared_sink,
                "delete  handler"
            ),

            AdminPublicRequest::ListRequest(request) => call_service!(
                request,
                super::list::handle_list_request(request, ctx.context().clone()),
                shared_sink,
                "list handler"
            ),
            AdminPublicRequest::UpdateMetadataRequest(request) => super::metadata::ClientMetadataController::handle_metadata_update(
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
