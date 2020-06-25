//!
//! # Describe Topic CLI
//!
//! CLI to describe Topics and their corresponding Partitions
//!

use structopt::StructOpt;

use flv_client::profile::ControllerTargetConfig;
use flv_client::profile::ControllerTargetInstance;
use crate::Terminal;
use crate::error::CliError;
use crate::OutputType;
use crate::tls::TlsConfig;
use crate::profile::InlineProfile;

use super::helpers::describe_kf_topics;
use super::helpers::describe_sc_topics;

// -----------------------------------
//  Parsed Config
// -----------------------------------

#[derive(Debug)]
pub struct DescribeTopicsConfig {
    pub topic_names: Vec<String>,
    pub output: OutputType,
}

// -----------------------------------
// CLI Options
// -----------------------------------

#[derive(Debug, StructOpt)]
pub struct DescribeTopicsOpt {
    /// Topic names
    #[structopt(short = "t", long = "topic", value_name = "string")]
    topics: Vec<String>,

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
    tls: TlsConfig,

    #[structopt(flatten)]
    profile: InlineProfile,
}

impl DescribeTopicsOpt {
    /// Validate cli options and generate config
    fn validate(self) -> Result<(ControllerTargetConfig, DescribeTopicsConfig), CliError> {
        let target_server = ControllerTargetConfig::possible_target(
            self.sc,
            #[cfg(kf)]
            self.kf.kf,
            #[cfg(not(foo))]
            None,
            self.tls.try_into_file_config()?,
            self.profile.profile,
        )?;

        // transfer config parameters
        let describe_topics_cfg = DescribeTopicsConfig {
            output: self.output.unwrap_or(OutputType::default()),
            topic_names: self.topics,
        };

        // return server separately from topic result
        Ok((target_server, describe_topics_cfg))
    }
}

// -----------------------------------
//  CLI Processing
// -----------------------------------

/// Process describe topic cli request
pub async fn process_describe_topics<O>(
    out: std::sync::Arc<O>,
    opt: DescribeTopicsOpt,
) -> Result<String, CliError>
where
    O: Terminal,
{
    let (target_server, cfg) = opt.validate()?;

    (match target_server.connect().await? {
        ControllerTargetInstance::Kf(client) => {
            describe_kf_topics(client, cfg.topic_names, cfg.output, out).await
        }
        ControllerTargetInstance::Sc(client) => {
            describe_sc_topics(client, cfg.topic_names, cfg.output, out).await
        }
    })
    .map(|_| format!(""))
    .map_err(|err| err.into())
}

// Query Kafka server for T

mod output {

    use std::io::ErrorKind;

    use prettytable::Row;
    use prettytable::cell;
    use prettytable::row;

    use flv_client::client::*;
    use flv_client::metadata::topic::TopicMetadata;

    use crate::OutputType;
    use crate::error::CliError;
    use crate::DescribeObjectHandler;
    use crate::{KeyValOutputHandler, TableOutputHandler};
    use crate::Terminal;

    // Connect to Kafka Controller and query server for topic
    pub async fn describe_sc_topics<O>(
        mut client: ScClient,
        topics: Vec<String>,
        output_type: OutputType,
        out: std::sync::Arc<O>,
    ) -> Result<(), CliError>
    where
        O: Terminal,
    {
        let topic_args = if topics.len() > 0 { Some(topics) } else { None };
        // query none for empty topic_names array
        let topics = client.topic_metadata(topic_args).await?;

        out.describe_objects(&topics, output_type)
    }

    impl DescribeObjectHandler for TopicMetadata {
        fn label() -> &'static str {
            "topic"
        }

        fn label_plural() -> &'static str {
            "topics"
        }

        fn is_ok(&self) -> bool {
            self.topic.is_some()
        }

        fn is_error(&self) -> bool {
            self.error.is_some()
        }

        /// validate topic
        fn validate(&self) -> Result<(), CliError> {
            let name = &self.name;
            if let Some(error) = self.error {
                Err(CliError::IoError(IoError::new(
                    ErrorKind::Other,
                    format!("topic '{}' {}", name, error.to_sentence()),
                )))
            } else if self.topic.is_none() {
                Err(CliError::IoError(IoError::new(
                    ErrorKind::Other,
                    format!("topic '{}', undefined error", name),
                )))
            } else {
                Ok(())
            }
        }
    }

    // -----------------------------------
    // Implement - TableOutputHandler
    // -----------------------------------

    impl TableOutputHandler for TopicMetadata {
        /// table header implementation
        fn header(&self) -> Row {
            row!["ID", "LEADER", "REPLICAS", "LIVE-REPLICAS",]
        }

        /// return errors in string format
        fn errors(&self) -> Vec<String> {
            vec![]
        }

        /// table content implementation
        fn content(&self) -> Vec<Row> {
            let mut rows: Vec<Row> = vec![];
            if let Some(topic) = &self.topic {
                if let Some(ref partitions) = topic.partition_map {
                    for partition in partitions {
                        rows.push(row![
                            r -> partition.id,
                            c -> partition.leader,
                            l -> format!("{:?}", partition.replicas),
                            l -> format!("{:?}", partition.live_replicas),
                        ]);
                    }
                }
            }

            rows
        }
    }

    // -----------------------------------
    // Implement - KeyValOutputHandler
    // -----------------------------------

    impl KeyValOutputHandler for TopicMetadata {
        /// key value hash map implementation
        fn key_values(&self) -> Vec<(String, Option<String>)> {
            let mut key_values = Vec::new();
            if let Some(topic) = &self.topic {
                let reason = if topic.reason.len() > 0 {
                    topic.reason.clone()
                } else {
                    "-".to_owned()
                };
                key_values.push(("Name".to_owned(), Some(self.name.clone())));
                key_values.push(("Type".to_owned(), Some(topic.type_label().to_string())));
                if topic.assigned_partitions.is_some() {
                    key_values.push((
                        "Assigned Partitions".to_owned(),
                        Some(topic.assigned_partitions.as_ref().unwrap().clone()),
                    ));
                }
                key_values.push(("Partition Count".to_owned(), Some(topic.partitions_str())));
                key_values.push((
                    "Replication Factor".to_owned(),
                    Some(topic.replication_factor_str()),
                ));
                key_values.push((
                    "Ignore Rack Assignment".to_owned(),
                    Some(topic.ignore_rack_assign_str().to_string()),
                ));
                key_values.push(("Status".to_owned(), Some(topic.status_label().to_string())));
                key_values.push(("Reason".to_owned(), Some(reason)));
                key_values.push(("Partition Map".to_owned(), None));
                key_values.push(("-----------------".to_owned(), None));
            }
            key_values
        }
    }

}