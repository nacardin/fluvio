
use flv_metadata::spu::*;
use super::*;

/// for group status, we have custom spu group spec
impl K8ExtendedSpec for SpuGroupSpec {
    type K8Spec  = K8SpuGroupSpec;
    type K8Status = Self::Status;
}
