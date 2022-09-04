use rhai::plugin::*;
use std::sync::{Arc, Mutex};

use crate::engine::state::EntityId;

#[derive(Clone, Debug)]
pub struct Entity {
    id: EntityId,
    // Back reference, so the entity can act on its own
    state: Arc<Mutex<crate::State>>,
}

impl Entity {
    pub(crate) fn new(id: EntityId, state: Arc<Mutex<crate::State>>) -> Self {
        Self { id, state }
    }
}

lazy_static::lazy_static! {
    pub(crate) static ref ENTITY_MODULE: rhai::Shared<rhai::Module> = rhai::Shared::new(rhai::exported_module!(plugin_entity));
}

#[export_module]
mod plugin_entity {
    pub type Entity = super::Entity;

    pub fn set_component(entity: &mut Entity, name: String, value: rhai::Dynamic) {
        let mut state = entity.state.lock().unwrap();
        let entity = state.entity_mut(entity.id).unwrap();
        entity.components.insert(name, value);
    }

    pub fn get_component(entity: &mut Entity, name: String) -> rhai::Dynamic {
        let state = entity.state.lock().unwrap();
        let entity = state.entity(entity.id).unwrap();
        entity
            .components
            .get(&name)
            .cloned()
            .unwrap_or(rhai::Dynamic::UNIT)
    }

    pub fn delete_component(entity: &mut Entity, name: String) {
        let mut state = entity.state.lock().unwrap();
        let entity = state.entity_mut(entity.id).unwrap();
        entity.components.remove(&name);
    }
}
