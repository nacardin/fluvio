//!
//! # Profile Configurations
//!
//! Stores configuration parameter retrieved from the default or custom profile file.
//!
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::convert::TryFrom;

use log::debug;

use flv_future_aio::net::tls::AllDomainConnector;

use crate::client::*;
use crate::ClientError;

use super::config::ConfigFile;
use super::tls::TlsConfig;

/// Logical configuration for Sc
pub struct ScConfig {
    addr: String,
    tls: Option<TlsConfig>,
}

impl ScConfig {
    /// create new Sc with optional address and tls, assume default profile
    pub fn new(addr_option: Option<String>, tls: Option<TlsConfig>) -> Result<Self, ClientError> {
        Self::new_with_profile(addr_option, tls, None)
    }

    // create new Sc with optional address and tls and can specify optional preferred profile
    pub fn new_with_profile(
        addr_option: Option<String>,
        tls: Option<TlsConfig>,
        profile: Option<String>,
    ) -> Result<Self, ClientError> {
        if let Some(addr) = addr_option {
            debug!("using custom target addr: {}", addr);
            Ok(Self { addr, tls })
        } else {
            // look up using profile
            let config_file = ConfigFile::load(None)?;
            if let Some(cluster) = config_file
                .config()
                .current_cluster_or_with_profile(profile.as_ref().map(|p| p.as_ref()))
            {
                debug!("looking up using profile: cluster addr {}", cluster.addr);
                Ok(Self {
                    addr: cluster.addr().to_owned(),
                    tls: cluster.tls.clone(),
                })
            } else {
                Err(IoError::new(ErrorKind::Other, "no matched cluster found").into())
            }
        }
    }

    pub async fn connect(self) -> Result<ScClient, ClientError> {
        let connector = match self.tls {
            None => AllDomainConnector::default_tcp(),
            Some(tls) => TryFrom::try_from(tls)?,
        };
        let config = ClientConfig::new(self.addr, connector);
        let client = config.connect().await?;
        debug!("connected to sc: {}", client.config().addr());
        Ok(ScClient::new(client))
    }
}
