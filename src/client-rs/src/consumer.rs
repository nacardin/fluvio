use kf_protocol::api::*;
use kf_protocol::message::fetch::FetchablePartitionResponse;


use crate::client::*;
use crate::ClientError;
use crate::params::*;

pub struct ConsumerClient {
    serial: SerialClient
}

impl ConsumerClient  {

    async fn fetch_logs_once(
        &mut self,
        offset_option: FetchOffset,
        option: FetchLogOption,
    ) -> Result<FetchablePartitionResponse<RecordSet>, ClientError> {

        todo!()
    }
}