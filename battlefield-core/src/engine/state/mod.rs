use crate::Map;
use rhai::Dynamic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod entity;

pub(crate) use entity::{Entity, EntityId};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    entities: Vec<Entity>,
    map: Value,
    data: HashMap<String, rhai::Dynamic>,
}

impl State {
    pub(crate) fn new(map: &Map) -> Self {
        State {
            entities: vec![],
            map: serde_json::to_value(map.data().unwrap()).unwrap(),
            data: HashMap::default(),
        }
    }

    pub(crate) fn set_data(&mut self, key: String, value: Dynamic) {
        self.data.insert(key, value);
    }

    pub(crate) fn get_data(&mut self, key: String) -> Option<&Dynamic> {
        self.data.get(&key)
    }

    pub(crate) fn spawn(&mut self) -> EntityId {
        let id = self
            .entities
            .last()
            .map(|entity| entity.id.next())
            .unwrap_or_else(EntityId::initial);
        self.entities.push(Entity::new(id));
        id
    }

    pub(crate) fn entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|entity| entity.id == id)
    }

    pub(crate) fn entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|entity| entity.id == id)
    }
}
