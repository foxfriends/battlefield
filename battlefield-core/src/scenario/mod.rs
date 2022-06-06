use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod location;
mod map;
mod module_config;
mod tile;

pub use location::*;
pub use map::*;
pub use module_config::*;
pub use tile::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    name: String,
    description: String,
    map: Map,
    #[serde(deserialize_with = "module_map")]
    pub(crate) modules: HashMap<String, ModuleConfig>,
}
