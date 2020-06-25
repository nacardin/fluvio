//!
//! # List All Spus CLI
//!
//! CLI tree and processing to list SPUs
//!

use structopt::StructOpt;

use flv_client::profile::ScConfig;
use flv_metadata::spu::SpuSpec;

use crate::error::CliError;
use crate::OutputType;
use crate::Terminal;
use crate::target::ClusterTarget;
use super::format_spu_response_output;

#[derive(Debug)]
pub struct ListSpusConfig {
    pub output: OutputType,
}

#[derive(Debug, StructOpt)]
pub struct ListSpusOpt {

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

impl ListSpusOpt {
    /// Validate cli options and generate config
    fn validate(self) -> Result<(ScConfig, OutputType), CliError> {

        let target_server = self.target.load()?;

        // return server separately from topic result
        Ok((target_server, self.output.unwrap_or_default()))
    }
}

// -----------------------------------
//  CLI Processing
// -----------------------------------

/// Process list spus cli request
pub async fn process_list_spus<O>(out: std::sync::Arc<O>, opt: ListSpusOpt) -> Result<(), CliError>
where
    O: Terminal,
{
    let (target_server, output) = opt.validate()?;

    let mut client = target_server.connect().await?;
    let mut admin = client.admin().await;

    let spus = admin.list::<SpuSpec>(vec![]).await?;

    // format and dump to screen
    format_spu_response_output(out, spus,output)?;
    Ok(())
}
