use rhai::plugin::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct State(crate::State);

impl State {
    pub(super) fn new(state: crate::State) -> Self {
        Self(state)
    }

    pub(super) fn into_inner(self) -> crate::State {
        self.0
    }

    fn set(&mut self, key: String, value: rhai::Dynamic) {
        self.0.set_data(key, value);
    }

    fn get(&mut self, key: String) -> rhai::Dynamic {
        self.0.get_data(key)
    }
}

lazy_static::lazy_static! {
    pub(crate) static ref STATE_MODULE: rhai::Shared<rhai::Module> = rhai::Shared::new(rhai::exported_module!(plugin_state));
}

#[export_module]
mod plugin_state {
    pub type State = Arc<Mutex<super::State>>;

    pub fn set_state(state: &mut State, key: String, value: rhai::Dynamic) {
        let mut state = state.lock().unwrap();
        state.set(key, value);
    }

    pub fn get_state(state: &mut State, key: String) -> rhai::Dynamic {
        let mut state = state.lock().unwrap();
        state.get(key)
    }
}
