use super::ModuleId;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Serialize, Debug)]
pub struct ModuleConfig {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) configuration: HashMap<String, Value>,
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
    pub(crate) configuration: HashMap<String, Value>,
}

impl ModuleConfig {
    fn from_version(name: String, version: String) -> Self {
        Self {
            name,
            version,
            configuration: HashMap::default(),
        }
    }

    fn from_repr(name: &str, repr: ModuleConfigRepr) -> Self {
        Self {
            name: repr.name.unwrap_or_else(|| name.to_owned()),
            version: repr.version,
            configuration: repr.configuration,
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
