use super::Entity;
use rhai::plugin::*;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub(crate) static ref STATE_MODULE: rhai::Shared<rhai::Module> = rhai::Shared::new(rhai::exported_module!(plugin_state));
}

pub type State = Arc<Mutex<crate::State>>;

#[allow(clippy::mut_mutex_lock)]
#[export_module]
mod plugin_state {
    pub type State = super::State;

    #[rhai_fn(index_set)]
    pub fn set_state(state: &mut State, key: String, value: rhai::Dynamic) {
        let mut state = state.lock().unwrap();
        state.set_data(key, value);
    }

    #[rhai_fn(index_get)]
    pub fn get_state(state: &mut State, key: String) -> rhai::Dynamic {
        let mut state = state.lock().unwrap();
        state.get_data(key).cloned().unwrap_or(rhai::Dynamic::UNIT)
    }

    pub fn spawn_entity(state: &mut State) -> Entity {
        let id = state.lock().unwrap().spawn();
        Entity::new(id, state.clone())
    }
}
