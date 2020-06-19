use crate::ClientError;
use crate::client::*;

pub use super::*;
pub struct KfConfig(String);

impl KfConfig {
    pub fn new(domain: String) -> Self {
        Self(domain)
    }

    pub async fn connect(self) -> Result<KfClient, ClientError> {
        let config = ClientConfig::with_addr(self.0);
        Ok(KfClient::new(config.connect().await?))
    }
}
