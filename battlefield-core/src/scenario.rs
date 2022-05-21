use crate::string_or_struct::string_or_struct;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::convert::Infallible;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TileId(String);

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TerritoryId(String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    tile_type: TileId,
    territory: Option<TerritoryId>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    name: String,
    description: String,
    #[serde(default)]
    properties: HashMap<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    tile_types: HashMap<TileId, Tile>,
    tiles: Vec<Vec<Location>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    version: String,
    #[serde(default)]
    configuration: HashMap<String, Value>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    name: String,
    description: String,
    map: Map,
    #[serde(deserialize_with = "module_map")]
    modules: HashMap<String, ModuleConfig>,
}

fn module_map<'de, D>(deserializer: D) -> Result<HashMap<String, ModuleConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "string_or_struct")] ModuleConfig);
    let v = HashMap::<String, Wrapper>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|(k, Wrapper(v))| (k, v)).collect())
}
