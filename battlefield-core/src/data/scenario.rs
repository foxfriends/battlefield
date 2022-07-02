use super::{module_map, ModuleConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Scenario {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) map: String,
    #[serde(deserialize_with = "module_map")]
    pub(crate) modules: HashMap<String, ModuleConfig>,
}

impl Scenario {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn modules(&self) -> impl Iterator<Item = (&str, &ModuleConfig)> {
        self.modules
            .iter()
            .map(|(name, config)| (name.as_str(), config))
    }
}
