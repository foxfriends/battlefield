use crate::data::{self, ModuleId};
use crate::{Command, Error, ErrorKind, State};
use serde_json::Value;
use std::collections::HashMap;

mod builder;
mod module;
mod scenario;

pub use builder::EngineBuilder;
pub use module::Module;
pub use scenario::{Scenario, ScenarioError};

#[derive(Default)]
pub struct Engine {
    scenarios: Vec<Scenario>,
    modules: Vec<Module>,
}

impl Engine {
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

    pub fn modules(
        &self,
    ) -> impl Iterator<Item = &Module> + DoubleEndedIterator + ExactSizeIterator {
        self.modules.iter()
    }

    pub fn commands(
        &self,
        scenario: &data::Scenario,
        state: &State,
    ) -> crate::Result<Vec<Command>> {
        let mut scope = rhai::Scope::new();
        scope.push_constant("scenario", scenario.clone());
        scope.push_constant("state", state.clone());

        let mut engine = rhai::Engine::new();
        engine.register_global_module(crate::runtime::MODULE.clone());
        let required_modules = scenario
            .modules()
            .map(|(name, config)| (config.id(), name))
            .collect::<HashMap<_, _>>();

        let modules = self
            .modules
            .iter()
            .filter(|module| module.is_valid())
            .filter(|module| required_modules.contains_key(&module.id()))
            .collect::<Vec<_>>();

        if required_modules.len() != modules.len() {
            let missing = required_modules
                .into_keys()
                .filter(|key| modules.iter().any(|module| module.id() == *key))
                .map(|key| key.to_string())
                .collect::<Vec<_>>();
            return Err(Error::internal(
                ErrorKind::ModuleNotFound,
                format!(
                    "{} required modules were not found: {}",
                    missing.len(),
                    missing.join(", ")
                ),
            ));
        }

        let mut commands = vec![];
        for module in modules {
            let name = required_modules.get(&module.id()).unwrap();
            engine.register_static_module(format!("battlefield::{name}"), module.ast().unwrap());
            let module_commands: rhai::Array = engine.eval_with_scope(
                &mut scope,
                &format!("battlefield::{name}::commands(scenario, state);"),
            )?;
            commands.extend(
                module_commands
                    .into_iter()
                    .map(|string| Ok(Command(string.into_string()?)))
                    .collect::<Result<Vec<Command>, &str>>()
                    .map_err(|type_name| crate::Error::internal(crate::ErrorKind::RuntimeError, format!("`commands` must return an array of strings, but it contained a {type_name}.")))?,
            );
        }
        Ok(commands)
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
