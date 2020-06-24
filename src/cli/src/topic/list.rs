//!
//! # List Topics CLI
//!
//! CLI tree and processing to list Topics
//!

use structopt::StructOpt;

use log::debug;


use flv_client::profile::ScConfig;
use crate::Terminal;
use crate::error::CliError;
use crate::OutputType;
use crate::target::ClusterTarget;
use flv_client::metadata::topic::TopicSpec;

use super::helpers::list_kf_topics;
use super::helpers::list_sc_topics;

// -----------------------------------
//  Parsed Config
// -----------------------------------

#[derive(Debug)]
pub struct ListTopicsConfig {
    pub output: OutputType,
}

// -----------------------------------
// CLI Options
// -----------------------------------

#[derive(Debug, StructOpt)]
pub struct ListTopicsOpt {
   
    /// Output
    #[structopt(
        short = "o",
        long = "output",
        value_name = "type",
        possible_values = &OutputType::variants(),
        case_insensitive = true,
    )]
    output: Option<OutputType>,

    #[structopt(flatten)]
    target: ClusterTarget,
}

impl ListTopicsOpt {
    /// Validate cli options and generate config
    fn validate(self) -> Result<(ScConfig, OutputType), CliError> {
        
        let target_server = self.target.load()?;

        Ok((target_server, self.output.unwrap_or_default()))
    }
}

// -----------------------------------
//  CLI Processing
// -----------------------------------

/// Process list topics cli request
pub async fn process_list_topics<O>(
    out: std::sync::Arc<O>,
    opt: ListTopicsOpt,
) -> Result<String, CliError>
where
    O: Terminal,
{
    let (target_server, output_type) = opt.validate()?;

    debug!("list topics {:#?} ", output_type);

    let mut client = target_server.connect().await?;
    let mut admin = client.admin().await;

    let topics = admin::<TopicSpec>::list().await?;
    list_topics(out, client, cfg.output).await?;
    Ok("".to_owned())
}
