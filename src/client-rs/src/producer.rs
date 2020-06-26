use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::io::Error as IoError;

use futures::io::AsyncWrite;

use crate::client::*;
use crate::ClientError;


/// interface to producer
pub struct ProducerClient {
    serial: SerialClient
}

impl ProducerClient {

    pub async fn send_record(&mut self, record: Vec<u8>) -> Result<(), ClientError> {
        todo!()
    }

}


impl AsyncWrite for ProducerClient{

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8])
            -> Poll<Result<usize,IoError>> {
        todo!()
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(),IoError>> {
        todo!()
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(),IoError>> {
        todo!()
    }

    
}