use crate::data::{self, ModuleId};

mod builder;
mod map;
mod module;
mod runtime;
mod scenario;
mod state;

pub use builder::EngineBuilder;
pub use map::Map;
pub use module::Module;
pub use scenario::{Scenario, ScenarioError};
pub use state::State;

#[derive(Default)]
pub struct Engine {
    maps: Vec<Map>,
    scenarios: Vec<Scenario>,
    modules: Vec<Module>,
}

impl Engine {
    pub fn scenario(&self, name: &str) -> Option<&Scenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.name() == name)
    }

    pub fn maps(&self) -> impl Iterator<Item = &Map> + DoubleEndedIterator + ExactSizeIterator {
        self.maps.iter()
    }

    pub fn scenarios(
        &self,
    ) -> impl Iterator<Item = &Scenario> + DoubleEndedIterator + ExactSizeIterator {
        self.scenarios.iter()
    }

    pub fn initialize(&self, scenario: &data::Scenario) -> crate::Result<State> {
        let map = self
            .maps
            .iter()
            .filter(|map| map.is_valid())
            .find(|map| map.name().unwrap() == scenario.map)
            .unwrap();
        let state = State::new(map);
        self.runtime_init(scenario, state)
    }

    pub fn modules(
        &self,
    ) -> impl Iterator<Item = &Module> + DoubleEndedIterator + ExactSizeIterator {
        self.modules.iter()
    }
}
