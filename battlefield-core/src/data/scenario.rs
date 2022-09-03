use super::{module_map, ModuleConfig};
use battlefield_api as api;
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

    pub fn module(&self, module: &str) -> Option<&ModuleConfig> {
        self.modules.get(module)
    }
}

impl From<Scenario> for api::Scenario {
    fn from(scenario: Scenario) -> Self {
        Self {
            name: scenario.name,
            description: scenario.description,
            map: scenario.map,
            modules: scenario
                .modules
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}
