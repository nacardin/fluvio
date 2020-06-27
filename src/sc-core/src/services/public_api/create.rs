use std::io::Error as IoError;

use log::debug;
use kf_protocol::api::*;
use k8_metadata_client::MetadataClient;

use sc_api::FlvStatus;
use sc_api::objects::*;

use super::*;

/// Handler for create topic request
pub async fn handle_create_request<C>(
    request: RequestMessage<CreateRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FlvStatus>, IoError>
where
    C: MetadataClient,
{
    let (header, req) = request.get_header_request();

    let dry_run = req.dry_run;
    let name = req.name;
    
    let status = match req.spec {
        AllCreatableSpec::Topic(topic) => {
            super::topic::handle_create_topics_request(name, dry_run, topic, &ctx).await?
        },
        AllCreatableSpec::SpuGroup(group) => {
            super::spg::handle_create_spu_group_request(name, group,dry_run, ctx).await?
        },
        AllCreatableSpec::CustomSpu(custom) => {
            super::spu::handle_register_custom_spu_request(name, custom, dry_run, ctx).await?
        }
    };


    Ok(ResponseMessage::from_header(&header, status))
}