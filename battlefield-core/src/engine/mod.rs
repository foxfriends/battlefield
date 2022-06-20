use crate::{data, Command, Error, ErrorKind, State};
use serde_json::Value;
use std::collections::HashMap;

mod builder;
mod module;
mod scenario;

pub use builder::EngineBuilder;
pub use module::{Module, ModuleId};
pub use scenario::{Scenario, ScenarioError};

#[derive(Default)]
pub struct Engine {
    scenarios: Vec<Scenario>,
    modules: HashMap<ModuleId, Module>,
}

impl Engine {
    fn require_module(&self, id: ModuleId) -> crate::Result<&Module> {
        self.modules.get(&id).ok_or_else(|| {
            Error::internal(ErrorKind::ModuleNotFound, format!("Module {id} not found"))
        })
    }

    pub fn scenario(&self, name: &str) -> Option<&Scenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.name() == name)
    }

    pub fn scenarios(
        &self,
    ) -> impl Iterator<Item = &Scenario> + DoubleEndedIterator + ExactSizeIterator {
        self.scenarios.iter()
    }

    pub fn modules(&self) -> impl Iterator<Item = &Module> + ExactSizeIterator {
        self.modules.values()
    }

    pub fn commands(
        &self,
        scenario: &data::Scenario,
        state: &State,
    ) -> crate::Result<Vec<Command>> {
        let mut scope = rhai::Scope::new();
        scope.push("commands", Vec::<Command>::new());
        scope.push_constant("scenario", scenario.clone());
        scope.push_constant("state", state.clone());

        let mut engine = rhai::Engine::new();
        for (name, config) in scenario.modules() {
            let module = self.require_module(config.id())?;
            engine.register_static_module(format!("battlefield::{name}"), module.ast().unwrap());
        }
        for (name, _) in scenario.modules() {
            engine.run_with_scope(&mut scope, &format!("battlefield::{name}::commands();"))?;
        }
        Ok(scope.get_value("commands").unwrap())
    }

    pub fn perform(
        &self,
        _command: Command,
        _scenario: &data::Scenario,
        _state: &mut State,
    ) -> crate::Result<Value> {
        Ok(Value::default())
    }
}
