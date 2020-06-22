
use sc_api::AdminRequest;
use kf_socket::KfSocketError;
use kf_socket::AllMultiplexerSocket;

use super::*;

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
    pub async fn admin(&mut self) -> ScAdminClient {
        ScAdminClient::new(SerialClient::new(
    self.socket.create_serial_socket().await,
    self.config.clone(),
    self.versions.clone()
        ))
    }

}


