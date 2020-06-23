//!
//! # Create Mange SPU Groups
//!
//! CLI tree to generate Create Managed SPU Groups
//!

use log::debug;
use structopt::StructOpt;

use flv_client::profile::ScConfig;
use flv_client::metadata::spg::SpuGroupSpec;

use crate::error::CliError;
use crate::target::ClusterTarget;

use super::helpers::group_config::GroupConfig;

// -----------------------------------
// CLI Options
// -----------------------------------

#[derive(Debug, StructOpt, Default)]
pub struct CreateManagedSpuGroupOpt {
    /// Managed SPU group name
    #[structopt(short = "n", long = "name", value_name = "string")]
    pub name: String,

    /// SPU replicas
    #[structopt(short = "l", long = "replicas")]
    pub replicas: u16,

    /// Minimum SPU id (default: 1)
    #[structopt(short = "i", long = "min-id", default_value="1")]
    pub min_id: i32,

    /// Rack name
    #[structopt(short = "r", long = "rack", value_name = "string")]
    pub rack: Option<String>,

    /// storage size
    #[structopt(short = "s", long = "size", value_name = "string")]
    pub storage: Option<String>,

    #[structopt(flatten)]
    target: ClusterTarget
}

impl CreateManagedSpuGroupOpt {
    /// Validate cli options. Generate target-server and create spu group config.
    fn validate(self) -> Result<(ScConfig, (String,SpuGroupSpec)), CliError> {
        let target_server = self.target.load()?;

        let grp_config = self
            .storage
            .map(|storage| GroupConfig::with_storage(storage));

        let group = (
            self.name,
            SpuGroupSpec {
                replicas: self.replicas,
                min_id: self.min_id,
                config: grp_config.map(|cf| cf.into()).unwrap_or_default(),
                rack: self.rack,
            }
        );
        // return server separately from config

        Ok((target_server, group))
    }
}

// -----------------------------------
//  CLI Processing
// -----------------------------------
pub async fn process_create_managed_spu_group(
    opt: CreateManagedSpuGroupOpt,
) -> Result<(), CliError> {
    let (target_server, (name,spec)) = opt.validate()?;

    debug!("creating spg: {}, spec: {:#?}", name,spec);

    let mut target = target_server.connect().await?;

    let mut admin = target.admin().await;

    admin.create(name,false,spec).await?;

    Ok(())
}
