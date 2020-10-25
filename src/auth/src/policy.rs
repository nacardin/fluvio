use std::fmt::Debug;

use futures_util::io::{AsyncRead, AsyncWrite};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};


use fluvio_controlplane_metadata::core::Spec;
use fluvio_socket::InnerFlvSocket;

use super::AuthError;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub enum TypeAction {
    Create,
    Read,
}

pub enum InstanceAction {
    Delete
}



#[async_trait]
pub trait AuthContext {

    /// check if any allow type specific action can be allowed
    async fn allow_type_action<S: Spec>(&self,action: TypeAction) -> Result<bool,AuthError>;

    /// check if specific instance of action can be permitted
    async fn allow_instance_action<S>(&self, action: InstanceAction, key: &S::IndexKey) -> Result<bool,AuthError>
        where S: Spec + Send,
             S::IndexKey: Sync;
    
}



#[async_trait]
pub trait Authorization
{

    type Stream: AsyncRead + AsyncWrite + Unpin + Send;
    type Context: AuthContext;

    /// create auth context
    async fn create_auth_context(&self, socket: &mut InnerFlvSocket<Self::Stream>
    ) -> Result<Self::Context, AuthError>;
        
}

