use super::TileId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TerritoryId(String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    tile_type: TileId,
    territory: Option<TerritoryId>,
}
