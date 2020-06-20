//!
//! # SPU Spec
//!
//! Interface to the SPU metadata spec in K8 key value store
//!
use serde::Deserialize;
use serde::Serialize;

use k8_obj_metadata::Crd;
use k8_obj_metadata::Spec;
use k8_obj_metadata::DefaultHeader;

use k8_obj_metadata::Env;
use k8_obj_metadata::TemplateSpec;
use crate::spu::EncryptionEnum;

use flv_types::defaults::SPU_PUBLIC_PORT;
use flv_types::defaults::SPU_PRIVATE_PORT;

use super::SPG_API;
use super::SpuGroupStatus;

impl Spec for SpuGroupSpec {
    type Status = SpuGroupStatus;
    type Header = DefaultHeader;
    fn metadata() -> &'static Crd {
        &SPG_API
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct SpuGroupSpec {
    pub template: TemplateSpec<SpuTemplate>,
    pub replicas: u16,
    #[serde(default)]
    pub min_id: i32
}


#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct SpuTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_endpoint: Option<SpuEndpointTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<SpuEndpointTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_svc: Option<ControllerEndPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<ReplicationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<StorageConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<Env>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpuEndpointTemplate {
    pub port: u16,
    pub encryption: EncryptionEnum,
}

impl SpuEndpointTemplate {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            encryption: EncryptionEnum::PLAINTEXT,
        }
    }

    pub fn default_public() -> Self {
        Self {
            port: SPU_PUBLIC_PORT,
            encryption: EncryptionEnum::PLAINTEXT,
        }
    }

    pub fn default_private() -> Self {
        Self {
            port: SPU_PRIVATE_PORT,
            encryption: EncryptionEnum::PLAINTEXT,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ControllerEndPoint {
    pub port: u16,
    pub hoste: String,
    pub encryption: EncryptionEnum,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationConfig {
    pub in_sync_replica_min: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl StorageConfig {
    pub fn log_dir(&self) -> String {
        self.log_dir.clone().unwrap_or("/tmp/fluvio".to_owned())
    }

    pub fn size(&self) -> String {
        self.size.clone().unwrap_or("1Gi".to_owned())
    }
}

#[cfg(feature = "flv")]
mod convert {

    use flv_metadata::spg::SpuGroupSpec as FlvSpgSpec;
    use flv_metadata::spg::GroupConfig as FlvGroupConfig;
    use flv_metadata::spg::StorageConfig as FlvStorageConfig;


    use crate::metadata::TemplateSpec;
    use super::*;

    impl From<FlvSpgSpec> for SpuGroupSpec {
        fn from(spec: FlvSpgSpec) -> Self {
           
            Self {
                replicas: spec.replicas,
                min_id: spec.min_id,
                template: TemplateSpec {
                    spec: SpuTemplate {
                        rack: spec.rack,
                        storage: spec.config.storage.map(|s| s.into()),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }
        }
    }

   impl From<FlvStorageConfig> for StorageConfig {
       fn from(storage: FlvStorageConfig) -> Self {
           Self {
               log_dir: storage.log_dir,
               size: storage.size
           }
       }
   }

    impl Into<FlvSpgSpec> for SpuGroupSpec {
        fn into(self) -> FlvSpgSpec {

            let min_id = self.min_id;
            let (replicas, template) = (self.replicas, self.template.spec);
            let (rack, storage) = (template.rack, template.storage.unwrap_or_default());
            FlvSpgSpec {
                replicas,
                min_id,
                rack,
                config: FlvGroupConfig {
                    storage: Some(storage.into()),
                    ..Default::default()
                }
               
            }
        }
    }

    impl Into<FlvStorageConfig> for StorageConfig {
        fn into(self) -> FlvStorageConfig {

            FlvStorageConfig {
                log_dir: self.log_dir,
                size: self.size
            }

        }
    }

    

}
