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

use kf_protocol::message::metadata::KfMetadataRequest;

use super::versions::ApiVersionsRequest;
use super::spu::*;
use super::topics::*;
use super::metadata::*;


use super::ScPublicApiKey;

#[derive(Debug, Encode)]
pub enum ScPublicRequest {
    // Mixed
    ApiVersionsRequest(RequestMessage<ApiVersionsRequest>),

    // Kafka
    KfMetadataRequest(RequestMessage<KfMetadataRequest>),

    // Fluvio - Topics
    FlvCreateTopicRequest(RequestMessage<FlvCreateTopicRequest>),
    FlvDeleteTopicRequest(RequestMessage<FlvDeleteTopicRequest>),
    FlvFetchTopicsRequest(RequestMessage<FlvFetchTopicsRequest>),
    FlvTopicCompositionRequest(RequestMessage<FlvTopicCompositionRequest>),

    // Fluvio - Spus
    FlvRegisterCustomSpuRequest(RequestMessage<FlvRegisterCustomSpuRequest>),
    FlvUnregisterCustomSpuRequest(RequestMessage<FlvUnregisterCustomSpuRequest>),
    FlvFetchSpusRequest(RequestMessage<FlvFetchSpusRequest>),

    FlvCreateSpuGroupRequest(RequestMessage<FlvCreateSpuGroupRequest>),
    FlvDeleteSpuGroupRequest(RequestMessage<FlvDeleteSpuGroupRequest>),
    FlvFetchSpuGroupsRequest(RequestMessage<FlvFetchSpuGroupsRequest>),

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

            //Kafka
            ScPublicApiKey::KfMetadata => api_decode!(Self, KfMetadataRequest, src, header),

            // Fluvio - Topics
            ScPublicApiKey::FlvCreateTopic => {
                api_decode!(Self, FlvCreateTopicRequest, src, header)
            }
            ScPublicApiKey::FlvDeleteTopic => {
                api_decode!(Self, FlvDeleteTopicRequest, src, header)
            }
            ScPublicApiKey::FlvFetchTopics => api_decode!(Self, FlvFetchTopicsRequest, src, header),
            ScPublicApiKey::FlvTopicComposition => {
                api_decode!(Self, FlvTopicCompositionRequest, src, header)
            }

            // Fluvio - Custom Spus / Spu Groups
            ScPublicApiKey::FlvRegisterCustomSpu => {
                api_decode!(Self, FlvRegisterCustomSpuRequest, src, header)
            }
            ScPublicApiKey::FlvUnregisterCustomSpu => {
                api_decode!(Self, FlvUnregisterCustomSpuRequest, src, header)
            }
            ScPublicApiKey::FlvFetchSpus => api_decode!(Self, FlvFetchSpusRequest, src, header),

            ScPublicApiKey::FlvCreateSpuGroup => {
                api_decode!(Self, FlvCreateSpuGroupRequest, src, header)
            }
            ScPublicApiKey::FlvDeleteSpuGroup => {
                api_decode!(Self, FlvDeleteSpuGroupRequest, src, header)
            }
            ScPublicApiKey::FlvFetchSpuGroups => {
                api_decode!(Self, FlvFetchSpuGroupsRequest, src, header)
            }

            ScPublicApiKey::FlvUpdateMetadata => {
                api_decode!(Self, UpdateMetadataRequest, src, header)
           }
        }
    }
}
