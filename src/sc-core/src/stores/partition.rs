

use flv_metadata::partition::store::*;
use flv_metadata::partition::*;

use super::*;

pub type PartitionAdminMd = PartitionMetadata<K8MetaContext>;
pub type PartitionAdminStore = PartitionLocalStore<K8MetaContext>;


use super::*;


impl K8ExtendedSpec for PartitionSpec {
    type K8Spec   = Self;
    type K8Status = Self::Status;
}
