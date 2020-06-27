
use log::{debug, trace};
use std::io::Error;


use k8_metadata_client::MetadataClient;
use kf_protocol::api::FlvErrorCode;
use sc_api::FlvStatus;


use super::PublicContext;

/// Handler for delete spu group request
pub async fn handle_delete_spu_group<C>(
    name: String,
    ctx: &PublicContext<C>,
) -> Result<FlvStatus, Error>
where
    C: MetadataClient,
{
    use flv_metadata::spg::K8SpuGroupSpec;

    debug!("delete spg group: {}",name);

    let status = match ctx.delete::<K8SpuGroupSpec>(&name).await {
        Ok(_) => FlvStatus::new_ok(name),
        Err(err) => {
              let error = Some(err.to_string());
              FlvStatus::new(name, FlvErrorCode::SpuError, error)
        }
    };

    trace!("flv delete spu group resp {:#?}", status);   

    Ok(status)
}


