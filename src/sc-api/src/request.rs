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
use super::spu::*;
use super::topics::*;
use super::metadata::*;


use super::ScPublicApiKey;

#[derive(Debug, Encode)]
pub enum ScPublicRequest {
    // Mixed
    ApiVersionsRequest(RequestMessage<ApiVersionsRequest>),

    //Topics
    CreateTopicRequest(RequestMessage<CreateTopicRequest>),
    DeleteTopicRequest(RequestMessage<DeleteTopicRequest>),
    FetchTopicsRequest(RequestMessage<FetchTopicsRequest>),
    TopicCompositionRequest(RequestMessage<TopicCompositionRequest>),

    // Spus
    RegisterCustomSpuRequest(RequestMessage<RegisterCustomSpuRequest>),
    UnregisterCustomSpuRequest(RequestMessage<UnregisterCustomSpuRequest>),
    FetchSpusRequest(RequestMessage<FetchSpusRequest>),

    CreateSpuGroupRequest(RequestMessage<CreateSpuGroupRequest>),
    DeleteSpuGroupRequest(RequestMessage<DeleteSpuGroupRequest>),
    FetchSpuGroupsRequest(RequestMessage<FetchSpuGroupsRequest>),

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
            ScPublicApiKey::CreateTopic => {
                api_decode!(Self, CreateTopicRequest, src, header)
            }
            ScPublicApiKey::DeleteTopic => {
                api_decode!(Self, DeleteTopicRequest, src, header)
            }
            ScPublicApiKey::FetchTopics => api_decode!(Self, FetchTopicsRequest, src, header),
            ScPublicApiKey::TopicComposition => {
                api_decode!(Self, TopicCompositionRequest, src, header)
            }

            // Fluvio - Custom Spus / Spu Groups
            ScPublicApiKey::RegisterCustomSpu => {
                api_decode!(Self, RegisterCustomSpuRequest, src, header)
            }
            ScPublicApiKey::UnregisterCustomSpu => {
                api_decode!(Self, UnregisterCustomSpuRequest, src, header)
            }
            ScPublicApiKey::FetchSpus => api_decode!(Self, FetchSpusRequest, src, header),

            ScPublicApiKey::CreateSpuGroup => {
                api_decode!(Self, CreateSpuGroupRequest, src, header)
            }
            ScPublicApiKey::DeleteSpuGroup => {
                api_decode!(Self, DeleteSpuGroupRequest, src, header)
            }
            ScPublicApiKey::FetchSpuGroups => {
                api_decode!(Self, FetchSpuGroupsRequest, src, header)
            }

            ScPublicApiKey::UpdateMetadata => {
                api_decode!(Self, UpdateMetadataRequest, src, header)
           }
        }
    }
}
