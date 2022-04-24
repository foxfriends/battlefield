use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

type TileId = String;

#[derive(Serialize, Deserialize)]
pub struct Tile {
    name: String,
    description: String,
    #[serde(default)]
    properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    tile_types: HashMap<TileId, Tile>,
    tiles: Vec<Vec<TileId>>,
}

#[derive(Serialize, Deserialize)]
pub struct Scenario {
    name: String,
    description: String,
    map: Map,
}
