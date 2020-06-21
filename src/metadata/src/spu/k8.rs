use k8_obj_metadata::Crd;
use k8_obj_metadata::Spec;
use k8_obj_metadata::DefaultHeader;


use super::SpuStatus;

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