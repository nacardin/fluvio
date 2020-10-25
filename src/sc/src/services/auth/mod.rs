pub mod basic;


//#[cfg(test)]
// mod test;


pub use common::*;

mod common {

    use std::sync::Arc;
    use std::fmt::Debug;

    use async_trait::async_trait;

    use fluvio_future::net::TcpStream; 
    use fluvio_auth::{ AuthContext, Authorization, TypeAction, InstanceAction };
    use fluvio_socket::InnerFlvSocket;
    use fluvio_controlplane_metadata::core::Spec;

    use crate::core::SharedContext;


    /// SC global context with authorization
    /// auth is trait object which contains global auth auth policy
    #[derive(Clone,Debug)]
    pub struct AuthGlobalContext<A>
    {
        pub global_ctx: SharedContext,
        pub auth: Arc<A>
    }

    impl <A> AuthGlobalContext<A>
    {

        pub fn new(global_ctx: SharedContext, auth: Arc<A>) -> Self {
            Self {
                global_ctx,
                auth
            }
        }
    }

    /// Authorization that allows anything
    /// Used for personal development
    #[derive(Debug,Clone)] 
    pub struct RootAuthorization {
    }

    #[async_trait]
    impl Authorization  for RootAuthorization {
        type Stream = TcpStream;
        type Context = RootAuthContext;

        async fn create_auth_context(&self, socket: &mut InnerFlvSocket<Self::Stream>) -> Result<Self::Context, std::io::Error> {
            Ok(RootAuthContext{})
        }
    }

    impl RootAuthorization {

        pub fn new() -> Self {
            Self{
            }
        }
    }

    #[derive(Debug)]
    pub struct RootAuthContext {

    }

    #[async_trait]
    impl AuthContext for RootAuthContext {

        async fn type_action_allowed<S: Spec>(&self,action: TypeAction) -> Result<bool,std::io::Error> {
            Ok(true)
        }
    
        /// check if specific instance of spec can be deleted
        async fn instance_action_allowed<S>(&self, action: InstanceAction, key: &S::IndexKey) -> Result<bool,std::io::Error>
        where S: Spec + Send,
             S::IndexKey: Sync
        {
            Ok(true)
        }
    

    }

    

    /// Auth Service Context, this hold individual context that is enough enforce auth
    /// for this service context
    pub struct AuthServiceContext<AC> {
        pub global_ctx: SharedContext,
        pub auth: AC
    }
    
    impl <AC>AuthServiceContext<AC> {
    
        pub fn new(global_ctx: SharedContext, auth: AC) -> Self {
            Self {
                global_ctx,
                auth
            }
        }
    }



    
}

