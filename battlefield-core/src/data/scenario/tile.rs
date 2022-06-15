use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TileId(String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    name: String,
    description: String,
    #[serde(default)]
    properties: HashMap<String, Value>,
}
