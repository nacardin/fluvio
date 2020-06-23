//!
//! # Create Custom SPUs
//!
//! CLI tree to generate Create Custom SPUs
//!

use std::convert::TryFrom;

use structopt::StructOpt;

use flv_util::socket_helpers::ServerAddress;
use flv_client::profile::ScConfig;
use sc_api::spu::CustomSpuSpec;

use crate::tls::TlsConfig;
use crate::error::CliError;
use crate::profile::InlineProfile;

#[derive(Debug)]
pub struct RegisterCustomSpuConfig {
    pub spec: CustomSpuSpec,
    pub name: String,
}

#[derive(Debug, StructOpt)]
pub struct RegisterCustomSpuOpt {
    /// SPU id
    #[structopt(short = "i", long = "id")]
    id: i32,

    /// SPU name
    #[structopt(short = "n", long = "name", value_name = "string")]
    name: Option<String>,

    /// Rack name
    #[structopt(short = "r", long = "rack", value_name = "string")]
    rack: Option<String>,

    /// Public server::port
    #[structopt(short = "p", long = "public-server", value_name = "host:port")]
    public_server: String,

    /// Private server::port
    #[structopt(short = "v", long = "private-server", value_name = "host:port")]
    private_server: String,

    /// Address of Streaming Controller
    #[structopt(short = "c", long = "sc", value_name = "host:port")]
    sc: Option<String>,

    #[structopt(flatten)]
    tls: TlsConfig,

    #[structopt(flatten)]
    profile: InlineProfile,
}

impl RegisterCustomSpuOpt {
    /// Validate cli options. Generate target-server and register custom spu config.
    fn validate(self) -> Result<(ScConfig, RegisterCustomSpuConfig), CliError> {
        // profile specific configurations (target server)
        let target_server = ScConfig::new_with_profile(
            self.sc,
            self.tls.try_into_file_config()?,
            self.profile.profile,
        )?;

        // register custom spu config
        let cfg = RegisterCustomSpuConfig {
            name: self.name.unwrap_or(format!("custom-spu-{}", self.id)),
            spec: CustomSpuSpec {
                id: self.id,
                public_endpoint: ServerAddress::try_from(self.public_server)?.into(),
                private_endpoint: ServerAddress::try_from(self.private_server)?.into(),
                rack: self.rack.clone(),
            },
        };

        // return server separately from config
        Ok((target_server, cfg))
    }
}

// -----------------------------------
//  CLI Processing
// -----------------------------------
pub async fn process_register_custom_spu(opt: RegisterCustomSpuOpt) -> Result<(), CliError> {
    let (target_server, cfg) = opt.validate()?;

    let mut sc = target_server.connect().await?;

    let admin = sc.admin().await;

    admin
        .create(cfg.name, false, cfg.spec)
        .await
        .map_err(|err| err.into())?;

    Ok(())
}
