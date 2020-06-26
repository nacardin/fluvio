//!
//! # API Requests
//!
//! Maps SC Api Requests with their associated Responses.
//!

use std::convert::TryInto;
use std::io::Error as IoError;

use log::trace;

use kf_protocol::bytes::Buf;

use kf_protocol::api::KfRequestMessage;
use kf_protocol::api::RequestHeader;
use kf_protocol::api::RequestMessage;

use kf_protocol::api::api_decode;
use kf_protocol::derive::Encode;

use super::versions::ApiVersionsRequest;
use super::metadata::*;
use super::objects::*;

use super::AdminPublicApiKey;

#[derive(Debug, Encode)]
pub enum AdminPublicRequest {
    // Mixed
    ApiVersionsRequest(RequestMessage<ApiVersionsRequest>),

    CreateRequest(RequestMessage<CreateRequest>),
    DeleteRequest(RequestMessage<DeleteRequest>),
    ListRequest(RequestMessage<ListRequest>),
    TopicCompositionRequest(RequestMessage<TopicCompositionRequest>),
    UpdateMetadataRequest(RequestMessage<UpdateMetadataRequest>),
}

impl Default for AdminPublicRequest {
    fn default() -> Self {
        Self::ApiVersionsRequest(RequestMessage::<ApiVersionsRequest>::default())
    }
}

impl KfRequestMessage for AdminPublicRequest {
    type ApiKey = AdminPublicApiKey;

    fn decode_with_header<T>(src: &mut T, header: RequestHeader) -> Result<Self, IoError>
    where
        Self: Default + Sized,
        Self::ApiKey: Sized,
        T: Buf,
    {
        trace!("decoding header: {:#?}", header);
        match header.api_key().try_into()? {
            AdminPublicApiKey::ApiVersion => api_decode!(Self, ApiVersionsRequest, src, header),

            AdminPublicApiKey::Create => api_decode!(Self, CreateRequest, src, header),
            AdminPublicApiKey::Delete => api_decode!(Self, DeleteRequest, src, header),
            AdminPublicApiKey::List => api_decode!(Self, ListRequest, src,header),
            AdminPublicApiKey::TopicComposition => {
                api_decode!(Self, TopicCompositionRequest, src, header)
            }
            AdminPublicApiKey::UpdateMetadata => api_decode!(Self, UpdateMetadataRequest, src, header),
        }
    }
}
