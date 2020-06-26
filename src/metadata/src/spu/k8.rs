use k8_obj_metadata::*;

use super::SpuStatus;
use super::SpuSpec;

const SPU_API: Crd = Crd {
    group: GROUP,
    version: V1,
    names: CrdNames {
        kind: "Spu",
        plural: "spus",
        singular: "spu",
    },
};


impl Spec for SpuSpec {
    type Status = SpuStatus;
    type Header = DefaultHeader;

    fn metadata() -> &'static Crd {
        &SPU_API
    }
}

impl Status for SpuStatus {}

use crate::core::K8ExtendedSpec;

impl K8ExtendedSpec for SpuSpec {
    type K8Spec  = Self;
    type K8Status = Self::Status;
}
