use crate::util::string_or_struct::string_or_struct;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub(crate) version: String,
    #[serde(default)]
    pub(crate) configuration: HashMap<String, Value>,
}

impl FromStr for ModuleConfig {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ModuleConfig {
            version: s.to_owned(),
            configuration: Default::default(),
        })
    }
}

pub(super) fn module_map<'de, D>(deserializer: D) -> Result<HashMap<String, ModuleConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "string_or_struct")] ModuleConfig);
    let v = HashMap::<String, Wrapper>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|(k, Wrapper(v))| (k, v)).collect())
}
