//! # List SPU Groups CLI
//!
//! CLI tree and processing to list SPU Groups
//!

use structopt::StructOpt;

use flv_client::profile::ScConfig;
use flv_client::metadata::spg::SpuGroupSpec;

use crate::output::OutputType;
use crate::error::CliError;
use crate::Terminal;
use crate::target::ClusterTarget;

use super::helpers::list_output::spu_group_response_to_output;

#[derive(Debug)]
pub struct ListSpuGroupsConfig {
    pub output: OutputType,
}

#[derive(Debug, StructOpt)]
pub struct ListManagedSpuGroupsOpt {
    /// Address of Streaming Controller
    #[structopt(short = "c", long = "sc", value_name = "host:port")]
    sc: Option<String>,

    /// Output
    #[structopt(
        short = "O",
        long = "output",
        value_name = "type",
        possible_values = &OutputType::variants(),
        case_insensitive = true
    )]
    output: Option<OutputType>,

    #[structopt(flatten)]
    target: ClusterTarget
}

impl ListManagedSpuGroupsOpt {
    /// Validate cli options and generate config
    fn validate(self) -> Result<(ScConfig, OutputType), CliError> {
        let target_server = self.target.load()?;

        
        Ok((target_server, self.output.unwrap_or_default()))
    }
}

/// Process list spus cli request
pub async fn process_list_managed_spu_groups<O: Terminal>(
    out: std::sync::Arc<O>,
    opt: ListManagedSpuGroupsOpt,
) -> Result<(), CliError> {
    let (target_server, output) = opt.validate()?;

    let mut client = target_server.connect().await?;
    let mut admin = client.admin().await;

    let lists = admin.list::<SpuGroupSpec>().await?;

    spu_group_response_to_output(out, lists,&output)
}
