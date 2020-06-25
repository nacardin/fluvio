use k8_obj_metadata::*;


use super::TopicStatus;
use super::TopicSpec;

const TOPIC_API: Crd = Crd {
    group: GROUP,
    version: V1,
    names: CrdNames {
        kind: "Topic",
        plural: "topics",
        singular: "topic",
    },
};

impl Spec for TopicSpec {
    type Status = TopicStatus;
    type Header = DefaultHeader;

    fn metadata() -> &'static Crd {
        &TOPIC_API
    }
}


impl Status for TopicStatus {}


    
use crate::core::K8ExtendedSpec;

impl K8ExtendedSpec for TopicSpec {
    type K8Spec = Self;
}


