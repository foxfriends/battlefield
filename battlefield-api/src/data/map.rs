use super::{Tile, TileId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Map {
    pub name: String,
    pub description: String,
    pub tiles: Vec<Vec<TileId>>,
    pub tile_types: HashMap<TileId, Tile>,
}
