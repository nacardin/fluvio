mod private_server;
mod internal_context;

pub use internal_context::InternalContext;

use std::sync::Arc;

use log::info;
use futures::channel::mpsc::Sender;

use private_server::ScInternalService;
use internal_api::InternalScKey;
use internal_api::InternalScRequest;
use internal_api::UpdateLrsRequest;
use kf_service::KfApiServer;

use crate::core::SharedContext;
use crate::controllers::conn_manager::*;

pub type SharedInternalContext = Arc<InternalContext>;

pub type InternalApiServer =
    KfApiServer<InternalScRequest, InternalScKey, SharedInternalContext, ScInternalService>;

// start server
pub fn create_internal_server(
    context: SharedContext,
    conn_mgr: SharedConnManager,
    conn_status_sender: Sender<SpuConnectionStatusChange>,
    lrs_sender: Sender<UpdateLrsRequest>,
) -> InternalApiServer {
    let addr = context.config().private_endpoint.clone();
    let ctx = InternalContext::new(context, conn_mgr, conn_status_sender, lrs_sender);
    info!("SC: starting internal services at: {}", addr);

    KfApiServer::new(addr, Arc::new(ctx), ScInternalService::new())
}
