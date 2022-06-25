use rhai::Dynamic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentType(String);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityId(u64);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    components: HashMap<ComponentType, Value>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct State {
    #[serde(default)]
    entities: Vec<Entity>,
    #[serde(default)]
    data: HashMap<String, Value>,
}

impl State {
    pub fn set_data(&mut self, key: String, value: Dynamic) {
        self.data.insert(key, serde_json::to_value(&value).unwrap());
    }

    pub fn get_data(&mut self, key: String) -> Dynamic {
        match self.data.get(&key) {
            Some(value) => rhai::serde::to_dynamic(value).unwrap(),
            None => rhai::Dynamic::UNIT,
        }
    }
}
