use super::{Tile, TileId};
use battlefield_api as api;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Map {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) tiles: Vec<Vec<TileId>>,
    pub(crate) tile_types: HashMap<TileId, Tile>,
}

impl From<Map> for api::Map {
    fn from(map: Map) -> Self {
        Self {
            name: map.name,
            description: map.description,
            tiles: map
                .tiles
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            tile_types: map
                .tile_types
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}
