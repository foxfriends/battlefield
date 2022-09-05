use battlefield_api as api;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TileId(String);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tile {
    name: String,
    description: String,
    #[serde(default)]
    properties: HashMap<String, Value>,
}

impl From<TileId> for api::TileId {
    fn from(id: TileId) -> Self {
        Self(id.0)
    }
}

impl From<Tile> for api::Tile {
    fn from(tile: Tile) -> Self {
        Self {
            name: tile.name,
            description: tile.description,
            properties: tile.properties,
        }
    }
}
