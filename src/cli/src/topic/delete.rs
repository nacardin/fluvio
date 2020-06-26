//!
//! # Delete Topics
//!
//! CLI tree to generate Delete Topics
//!

use log::debug;
use structopt::StructOpt;

use flv_client::config::ScConfig;
use flv_client::metadata::topic::TopicSpec;
use crate::error::CliError;
use crate::target::ClusterTarget;

#[derive(Debug, StructOpt)]
pub struct DeleteTopicOpt {
    /// Topic name
    #[structopt(short = "t", long = "topic", value_name = "string")]
    topic: String,

    #[structopt(flatten)]
    target: ClusterTarget,
}

impl DeleteTopicOpt {
    /// Validate cli options. Generate target-server and delete-topic configuration.
    fn validate(self) -> Result<(ScConfig, String), CliError> {
        let target_server = self.target.load()?;

        // return server separately from config
        Ok((target_server, self.topic))
    }
}

// -----------------------------------
//  CLI Processing
// -----------------------------------

/// Process delete topic cli request
pub async fn process_delete_topic(opt: DeleteTopicOpt) -> Result<String, CliError> {
    let (target_server, name) = opt.validate()?;

    debug!("deleting topic: {}", name);

    let mut client = target_server.connect().await?;
    let mut admin = client.admin().await;
    admin.delete::<TopicSpec, _>(&name).await?;
    Ok(format!("topic \"{}\" deleted", name))
}
