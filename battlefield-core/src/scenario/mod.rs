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
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) map: Map,
    #[serde(deserialize_with = "module_map")]
    pub(crate) modules: HashMap<String, ModuleConfig>,
}
