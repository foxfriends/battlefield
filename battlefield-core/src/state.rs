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
}
