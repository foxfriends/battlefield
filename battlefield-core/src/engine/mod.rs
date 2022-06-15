use crate::util::flatten::Flatten;
use crate::{data, Command, Error, ErrorKind, State};
use serde_json::Value;
use std::collections::HashMap;

mod builder;
mod module;
mod scenario;

pub use builder::EngineBuilder;
pub(crate) use module::{Module, ModuleId};
pub(crate) use scenario::Scenario;

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

    pub fn scenario(&self, name: &str) -> Option<&data::Scenario> {
        self.scenarios
            .iter()
            .filter_map(|scenario| scenario.data())
            .find(|scenario| scenario.name == name)
    }

    pub fn commands(
        &self,
        scenario: &data::Scenario,
        state: &State,
    ) -> crate::Result<Vec<Command>> {
        let scenario = self
            .scenarios
            .iter()
            .find(|s| s.is(scenario))
            .ok_or_else(|| {
                crate::Error::internal(
                    crate::ErrorKind::ScenarioNotFound,
                    format!("Scenario {} not found", scenario.name),
                )
            })?;
        scenario
            .modules()
            .map(|module_config| {
                let module = self.require_module(module_config.id())?;
                Ok(module.commands(scenario, state))
            })
            .collect::<crate::Result<Flatten<Vec<Command>>>>()
            .map(|f| f.0)
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
