use log::debug;
use std::io::Error;

use kf_protocol::api::FlvErrorCode;
use kf_protocol::api::{RequestMessage, ResponseMessage};

use flv_metadata::spg::K8SpuGroupSpec;
use k8_metadata_client::MetadataClient;

use sc_api::spu::*;
use sc_api::FlvStatus;

use super::PublicContext;

pub async fn handle_fetch_spu_groups_request<C>(
    request: RequestMessage<FetchSpuGroupsRequest>,
    ctx: &PublicContext<C>,
) -> Result<ResponseMessage<FetchSpuGroupsResponse>, Error>
where
    C: MetadataClient,
{
    let mut response = FetchSpuGroupsResponse::default();

    match ctx.retrieve_items::<K8SpuGroupSpec>().await {
        Ok(k8_list) => {
            debug!("fetched: {} spgs", k8_list.items.len());

            response.spu_groups = k8_list.items.into_iter()
                .map(|k8_obj| {
                    FetchSpuGroup {
                        name: k8_obj.metadata.name,
                        spec: k8_obj.spec.into(),
                        status: k8_obj.status.into()
                    }
                })
                .collect();
            
        }
        Err(err) => {
            let error = Some(err.to_string());
            response.error =
            FlvStatus::new("error".to_owned(), FlvErrorCode::SpuError, error);
        }
    }

    Ok(request.new_response(response))
}
