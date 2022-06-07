use crate::module::{Module, ModuleId};
use crate::util::flatten::Flatten;
use crate::{Command, Scenario, State};
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
    fn require_module(&self, name: &str, version: &str) -> anyhow::Result<&Module> {
        let id = ModuleId::new(name.to_owned(), version.to_owned());
        self.modules
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Module {id} not found"))
    }

    pub fn scenario(&self, name: &str) -> Option<&Scenario> {
        self.scenarios.iter().find(|scenario| scenario.name == name)
    }

    pub fn commands(&self, scenario: &Scenario, state: &State) -> anyhow::Result<Vec<Command>> {
        scenario
            .modules
            .iter()
            .map(|(name, config)| {
                let module = self.require_module(name, &config.version)?;
                Ok(module.commands(scenario, state))
            })
            .collect::<anyhow::Result<Flatten<Vec<Command>>>>()
            .map(|f| f.0)
    }

    pub fn perform(
        &self,
        _command: Command,
        _scenario: &Scenario,
        _state: &mut State,
    ) -> anyhow::Result<Value> {
        Ok(Value::default())
    }
}
