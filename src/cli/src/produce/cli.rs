//!
//! # Produce CLI
//!
//! CLI command for Produce operation
//!

use std::path::PathBuf;

use structopt::StructOpt;

use flv_client::ClusterConfig;

use crate::target::ClusterTarget;
use crate::error::CliError;

// -----------------------------------
//  Parsed Config
// -----------------------------------

/// Produce log configuration parameters
#[derive(Debug)]
pub struct ProduceLogConfig {
    pub topic: String,
    pub partition: i32,
    pub continuous: bool,
    pub records_form_file: Option<FileRecord>,
}

#[derive(Debug)]
pub enum FileRecord {
    Lines(PathBuf),
    Files(Vec<PathBuf>),
}

// -----------------------------------
// CLI Options
// -----------------------------------

#[derive(Debug, StructOpt)]
pub struct ProduceLogOpt {
    /// Topic name
    #[structopt(value_name = "topic")]
    pub topic: String,

    /// Partition id
    #[structopt(
        short = "p",
        long = "partition",
        value_name = "integer",
        default_value = "0"
    )]
    pub partition: i32,

    /// Send messages in an infinite loop
    #[structopt(short = "C", long = "continuous")]
    pub continuous: bool,

    /// Each line is a Record
    #[structopt(
        short = "l",
        long = "record-per-line",
        value_name = "filename",
        parse(from_os_str)
    )]
    record_per_line: Option<PathBuf>,

    /// Entire file is a Record (multiple)
    #[structopt(
        short = "r",
        long = "record-file",
        value_name = "filename",
        parse(from_os_str),
        conflicts_with = "record_per_line"
    )]
    record_file: Vec<PathBuf>,

    #[structopt(flatten)]
    target: ClusterTarget,
}

impl ProduceLogOpt {
    /// Validate cli options. Generate target-server and produce log configuration.
    pub fn validate(self) -> Result<(ClusterConfig, ProduceLogConfig), CliError> {
        let target_server = self.target.load()?;

        // generate file record
        let records_from_file = if let Some(record_per_line) = self.record_per_line {
            Some(FileRecord::Lines(record_per_line.clone()))
        } else if self.record_file.len() > 0 {
            Some(FileRecord::Files(self.record_file.clone()))
        } else {
            None
        };

        // produce log specific configurations
        let produce_log_cfg = ProduceLogConfig {
            topic: self.topic,
            partition: self.partition,
            records_form_file: records_from_file,
            continuous: self.continuous,
        };

        // return server separately from config
        Ok((target_server, produce_log_cfg))
    }
}
