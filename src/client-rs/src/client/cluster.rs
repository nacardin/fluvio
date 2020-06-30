
use log::debug;
use log::trace;

use kf_socket::AllMultiplexerSocket;

use crate::admin::AdminClient;
use crate::Producer;
use crate::Consumer;
use crate::ClientError;

use super::*;

/// Gate way to Sc
/// All other clients are constructed from here
pub struct ClusterClient {
    socket: AllMultiplexerSocket,
    config: ClientConfig,
    versions: Versions
}

impl ClusterClient {
    pub(crate) fn new(client: RawClient) -> Self {

        let (socket, config, versions) = client.split();
        Self {
            socket: AllMultiplexerSocket::new(socket),
            config,
            versions
        }
    }

    async fn create_serial_client(&mut self) -> SerialClient {
        SerialClient::new(
            self.socket.create_serial_socket().await,
            self.config.clone(),
            self.versions.clone()
        )
    }

    /// create new admin client
    pub async fn admin(&mut self) -> AdminClient {
        AdminClient::new(self.create_serial_client().await)
    }

    /// create new producer for topic/partition
    pub async fn producer(
        &mut self,
        topic: &str,
        partition: i32,
    ) -> Result<Producer, ClientError> {

       Ok(Producer::new(self.create_serial_client().await,topic,partition))

    }


    /// create new consumer for topic/partition
    pub async fn consumer(
        &mut self,
        topic: &str,
        partition: i32,
    ) -> Result<Consumer, ClientError> {

        Ok(Consumer::new(self.create_serial_client().await, topic, partition))

    }

    /// start watch on metadata
    /// first, it get current metadata then wait for update
    pub async fn start_metadata_watch(
        &mut self
    ) -> Result<(), ClientError> {

        use std::time::Duration;

        use kf_protocol::api::RequestMessage;
        use flv_api_sc::metadata::WatchMetadataRequest;

        let req_msg = RequestMessage::new_request(WatchMetadataRequest::default());

        let mut metadata_async_response = self.socket.send_with_async_response(req_msg, 10).await?;        

        let full_metadata = metadata_async_response.next_timeout(Duration::from_secs(5)).await?;

        debug!("receives full metadata");

        trace!("metadata: {:#?}",full_metadata);
        
        Ok(())
        
    }
}


