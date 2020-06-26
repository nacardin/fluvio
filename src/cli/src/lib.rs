mod common;
mod error;
mod consume;
mod produce;
mod root_cli;
mod spu;
mod topic;
mod output;
mod profile;
mod tls;
mod cluster;
mod group;
mod custom;

pub use self::error::CliError;
pub use self::root_cli::run_cli;

use output::Terminal;
use output::*;

const VERSION: &'static str = include_str!("VERSION");

#[macro_export]
macro_rules! t_println {
    ($out:expr,$($arg:tt)*) => ( $out.println(&format!($($arg)*)))
}

#[macro_export]
macro_rules! t_print_cli_err {
    ($out:expr,$x:expr) => {
        t_println!($out, "\x1B[1;31merror:\x1B[0m {}", $x);
    };
}

mod target {

    use structopt::StructOpt;

    use flv_client::ClusterConfig;
    use crate::tls::TlsConfig;
    use crate::CliError;
    use crate::profile::InlineProfile;

    /// server configuration
    #[derive(Debug, StructOpt, Default)]
    pub struct ClusterTarget {
        /// address of cluster
        #[structopt(short = "c", long, value_name = "host:port")]
        pub cluster: Option<String>,

        #[structopt(flatten)]
        tls: TlsConfig,

        #[structopt(flatten)]
        profile: InlineProfile,
    }

    impl ClusterTarget {
        /// try to create sc config
        pub fn load(&self) -> Result<ClusterConfig, CliError> {

            // check case when inline profile is used
            if let Some(profile) = self.profile.profile {
                if self.cluster.is_some() {
                    Err(CliError::InvalidArg("cluster addr is not valid when profile is used"))
                } else if self.tls.is_some() {
                    Err(CliError::InvalidArg("tls is not valid when profile is is used"))
                } else {
                    ClusterConfig::lookup_profile(profile)
                }
               
            } else {
                // check if cluster address is used
                if let Some(cluster) = self.cluster {
                    Ok(ClusterConfig::new(cluster,self.tls))
                } else {
                    // check if tls is used
                    if self.tls.is_some() {
                        Err(CliError::InvalidArg("tls is only valid if cluster addr is used"))
                    } else {
                        ClusterConfig::lookup_profile(None)
                    }
                }
            }
            
        }
    }
}
