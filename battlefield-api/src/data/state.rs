use super::Entity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub entities: Vec<Entity>,
    pub map: serde_json::Value,
    pub data: HashMap<String, serde_json::Value>,
}
