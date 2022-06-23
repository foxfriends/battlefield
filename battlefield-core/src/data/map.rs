use super::{Location, Tile, TileId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Map {
    tile_types: HashMap<TileId, Tile>,
    tiles: Vec<Vec<Location>>,
}
