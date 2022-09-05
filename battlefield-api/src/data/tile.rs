use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TileId(pub String);

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Tile {
    pub name: String,
    pub description: String,
    pub properties: HashMap<String, Value>,
}
