use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
pub struct Scenario {
    name: String,
    description: String,
    map: Map,
}
