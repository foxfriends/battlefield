use super::ModuleId;
use battlefield_api as api;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Debug)]
pub struct ModuleConfig {
    pub(crate) name: String,
    pub(crate) version: String,
    #[serde(default)]
    pub(crate) config: toml::value::Table,
}

impl ModuleConfig {
    pub fn id(&self) -> ModuleId {
        ModuleId::new(self.name.clone(), self.version.clone())
    }
}

#[derive(Clone, Deserialize)]
pub struct ModuleConfigRepr {
    pub(crate) name: Option<String>,
    pub(crate) version: String,
    #[serde(default)]
    pub(crate) config: toml::value::Table,
}

impl ModuleConfig {
    fn from_version(name: String, version: String) -> Self {
        Self {
            name,
            version,
            config: Default::default(),
        }
    }

    fn from_repr(name: &str, repr: ModuleConfigRepr) -> Self {
        Self {
            name: repr.name.unwrap_or_else(|| name.to_owned()),
            version: repr.version,
            config: repr.config,
        }
    }
}

pub(super) fn module_map<'de, D>(deserializer: D) -> Result<HashMap<String, ModuleConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Wrapper {
        Version(String),
        Config(ModuleConfigRepr),
    }

    let v = HashMap::<String, Wrapper>::deserialize(deserializer)?;
    Ok(v.into_iter()
        .map(|(name, wrapper)| {
            let config = match wrapper {
                Wrapper::Version(version) => ModuleConfig::from_version(name.clone(), version),
                Wrapper::Config(repr) => ModuleConfig::from_repr(&name, repr),
            };
            (name, config)
        })
        .collect())
}

impl From<ModuleConfig> for api::Module {
    fn from(module: ModuleConfig) -> Self {
        Self {
            name: module.name,
            version: module.version,
            config: serde_json::from_str(&serde_json::to_string(&module.config).unwrap()).unwrap(),
        }
    }
}
