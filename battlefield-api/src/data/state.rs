use super::{Entity, Map};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct State {
    pub entities: Vec<Entity>,
    pub map: Map,
    pub data: HashMap<String, serde_json::Value>,
}
