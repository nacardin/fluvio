

use flv_metadata::partition::store::*;
use flv_metadata::partition::*;

use super::*;

pub type K8PartitionMd = PartitionMetadata<K8MetaContext>;
type K8PartitionLocalStore = PartitionLocalStore<K8MetaContext>;

pub type PartitionAdminStore = K8PartitionLocalStore;


use super::*;


impl K8ExtendedSpec for PartitionSpec {
    type K8Spec   = Self;
    type K8Status = Self::Status;
}
