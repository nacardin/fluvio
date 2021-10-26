use fluvio_controlplane_metadata::smartmodule::SmartModule;
use fluvio_types::SmartModuleId;

use crate::core::Spec;
use crate::core::LocalStore;

impl Spec for SmartModule {
    const LABEL: &'static str = "SmartModule";

    type Key = SmartModuleId;

    fn key(&self) -> &Self::Key {
        &self.name
    }

    fn key_owned(&self) -> Self::Key {
        self.name.clone()
    }
}

pub type SmartModuleLocalStore = LocalStore<SmartModule>;

impl SmartModuleLocalStore {
    #[cfg(test)]
    pub fn indexed_by_id(&self) -> std::collections::BTreeMap<SmartModuleId, SmartModule> {
        let mut map: std::collections::BTreeMap<SmartModuleId, SmartModule> = std::collections::BTreeMap::new();

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

