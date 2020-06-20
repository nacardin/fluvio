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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<i32>,
}

impl SpuGroupSpec {
    pub fn min_id(&self) -> i32 {
        self.min_id.unwrap_or(0)
    }
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



#[cfg(feature ="kf")]
mod convert {

    use flv_metadata::spg::SpgSpec as FlvSpgSpec;

    use crate::metadata::TemplateSpec;
    use crate::metadata::K8Obj;
    use super::*;

impl Into<(String, SpuGroupSpec, SpuGroupStatus)> for FlvFetchSpuGroup {
    fn into(self) -> (String, SpuGroupSpec, SpuGroupStatus) {
        (
            self.name,
            SpuGroupSpec {
                replicas: self.replicas,
                min_id: Some(self.min_id),
                template: TemplateSpec {
                    spec: SpuTemplate {
                        rack: self.rack,
                        storage: Some(StorageConfig {
                            size: Some(self.size),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            SpuGroupStatus {
                resolution: self.resolution.into(),
                ..Default::default()
            },
        )
    }
}

impl From<K8Obj<SpuGroupSpec>> for FlvFetchSpuGroup {
    fn from(item: K8Obj<SpuGroupSpec>) -> Self {
        let (name, spec, status) = (item.metadata.name, item.spec, item.status);
        let min_id = spec.min_id();
        let (replicas, template) = (spec.replicas, spec.template.spec);
        let (rack, storage) = (template.rack, template.storage.unwrap_or_default());
        Self {
            name,
            replicas,
            min_id,
            rack,
            size: storage.size(),
            resolution: status.resolution.into(),
            reason: None,
        }
    }
}

impl From<SpuGroupStatusResolution> for FlvSpuGroupResolution {
    fn from(res: SpuGroupStatusResolution) -> Self {
        match res {
            SpuGroupStatusResolution::Init => FlvSpuGroupResolution::Init,
            SpuGroupStatusResolution::Invalid => FlvSpuGroupResolution::Invalid,
            SpuGroupStatusResolution::Reserved => FlvSpuGroupResolution::Reserved,
        }
    }
}

impl Into<SpuGroupStatusResolution> for FlvSpuGroupResolution {
    fn into(self) -> SpuGroupStatusResolution {
        match self {
            Self::Init => SpuGroupStatusResolution::Init,
            Self::Invalid => SpuGroupStatusResolution::Invalid,
            Self::Reserved => SpuGroupStatusResolution::Reserved,
        }
    }
}



}