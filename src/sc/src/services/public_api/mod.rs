mod public_server;
mod spg;
mod spu;
mod topic;
mod partition;
mod api_version;
mod create;
mod delete;
mod list;
mod watch;

pub use server::start_public_server;

mod server {

    use std::fmt::Debug;

    use tracing::info;
    use tracing::instrument;

    use fluvio_service::FlvApiServer;
    use fluvio_auth::Authorization;

    use crate::services::auth::AuthGlobalContext;
    use super::public_server::PublicService;

    /// create public server
    pub fn start_public_server<A: Authorization>(ctx: AuthGlobalContext<A>)
    {
        let addr = ctx.config().public_endpoint.clone();
        info!("start public api service");
        let server = FlvApiServer::new(addr, ctx, PublicService::new());
        server.run();
    }
}
