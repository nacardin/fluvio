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

use super::ScPublicApiKey;

#[derive(Debug, Encode)]
pub enum ScPublicRequest {
    // Mixed
    ApiVersionsRequest(RequestMessage<ApiVersionsRequest>),

    CreateRequest(RequestMessage<CreateRequest>),

    TopicCompositionRequest(RequestMessage<TopicCompositionRequest>),
    UpdateMetadataRequest(RequestMessage<UpdateMetadataRequest>),
}

impl Default for ScPublicRequest {
    fn default() -> Self {
        Self::ApiVersionsRequest(RequestMessage::<ApiVersionsRequest>::default())
    }
}

impl KfRequestMessage for ScPublicRequest {
    type ApiKey = ScPublicApiKey;

    fn decode_with_header<T>(src: &mut T, header: RequestHeader) -> Result<Self, IoError>
    where
        Self: Default + Sized,
        Self::ApiKey: Sized,
        T: Buf,
    {
        trace!("decoding header: {:#?}", header);
        match header.api_key().try_into()? {
            // Mixed
            ScPublicApiKey::ApiVersion => api_decode!(Self, ApiVersionsRequest, src, header),

            // Fluvio - Topics
            ScPublicApiKey::Create => {
                api_decode!(Self, CreateRequest, src, header)
            },

            ScPublicApiKey::TopicComposition => {
                api_decode!(Self, TopicCompositionRequest, src, header)
            },

            ScPublicApiKey::UpdateMetadata => {
                api_decode!(Self, UpdateMetadataRequest, src, header)
           }
        }
    }
}
