

use kf_socket::AllMultiplexerSocket;
use crate::admin::AdminClient;
use crate::producer::ProducerClient;
use crate::consumer::ConsumerClient;
use crate::ClientError;

use super::*;

/// Gate way to Sc
/// All other clients are constructed from here
pub struct ScClient {
    socket: AllMultiplexerSocket,
    config: ClientConfig,
    versions: Versions
}

impl ScClient {
    pub(crate) fn new(client: RawClient) -> Self {

        let (socket, config, versions) = client.split();
        Self {
            socket: AllMultiplexerSocket::new(socket),
            config,
            versions
        }
    }

    /// create new admin client
    pub async fn admin(&mut self) -> AdminClient {
        AdminClient::new(SerialClient::new(
    self.socket.create_serial_socket().await,
    self.config.clone(),
    self.versions.clone()
        ))
    }


    async fn producer(
        &mut self,
        topic: &str,
        partition: i32,
    ) -> Result<ProducerClient, ClientError> {

        panic!("not yet implemented");

    }

    async fn consumer(
        &mut self,
        topic: &str,
        partition: i32,
    ) -> Result<ConsumerClient, ClientError> {

        panic!("not yet implemented");

    }
}


