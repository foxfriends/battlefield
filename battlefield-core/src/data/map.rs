use super::{Tile, TileId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Map {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) tiles: Vec<Vec<TileId>>,
    pub(crate) tile_types: HashMap<TileId, Tile>,
}
