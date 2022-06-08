use crate::module::{Module, ModuleId};
use crate::util::flatten::Flatten;
use crate::{Command, Error, ErrorKind, Scenario, State};
use serde_json::Value;
use std::collections::HashMap;

mod builder;

pub use builder::EngineBuilder;

// TODO: make this a real thing
#[derive(Copy, Clone, Debug)]
pub struct ModuleNotFound;

#[derive(Default)]
pub struct Engine {
    scenarios: Vec<Scenario>,
    modules: HashMap<ModuleId, Module>,
}

impl Engine {
    fn require_module(&self, name: &str, version: &str) -> crate::Result<&Module> {
        let id = ModuleId::new(name.to_owned(), version.to_owned());
        self.modules.get(&id).ok_or_else(|| {
            Error::internal(ErrorKind::ModuleNotFound, format!("Module {id} not found"))
        })
    }

    pub fn scenario(&self, name: &str) -> Option<&Scenario> {
        self.scenarios.iter().find(|scenario| scenario.name == name)
    }

    pub fn commands(&self, scenario: &Scenario, state: &State) -> crate::Result<Vec<Command>> {
        scenario
            .modules
            .iter()
            .map(|(name, config)| {
                let module = self.require_module(name, &config.version)?;
                Ok(module.commands(scenario, state))
            })
            .collect::<crate::Result<Flatten<Vec<Command>>>>()
            .map(|f| f.0)
    }

    pub fn perform(
        &self,
        _command: Command,
        _scenario: &Scenario,
        _state: &mut State,
    ) -> crate::Result<Value> {
        Ok(Value::default())
    }
}
