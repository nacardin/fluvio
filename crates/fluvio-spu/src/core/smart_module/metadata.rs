use fluvio_controlplane_metadata::smartmodule::SmartModuleSpec;
use fluvio_types::SmartModuleId;

use crate::core::Spec;
use crate::core::LocalStore;

impl Spec for SmartModuleSpec {
    const LABEL: &'static str = "SmartModule";

    type Key = SmartModuleId;

    fn key(&self) -> &Self::Key {
        &self.id
    }

    fn key_owned(&self) -> Self::Key {
        self.id.clone()
    }
}

pub type SmartModuleLocalStore = LocalStore<SmartModuleSpec>;

impl SmartModuleLocalStore {
    #[cfg(test)]
    pub fn indexed_by_id(&self) -> std::collections::BTreeMap<SmartModuleId, SmartModuleSpec> {
        let mut map: std::collections::BTreeMap<SmartModuleId, SmartModuleSpec> = std::collections::BTreeMap::new();

        for sm in self.inner_store().read().values() {
            map.insert(sm.id, sm.clone());
        }

        map
    }

    #[cfg(test)]
    pub fn bulk_add(self, sms: Vec<i32>) -> Self {
        for id in sms {
            self.insert(id.into());
        }
        self
    }
}

