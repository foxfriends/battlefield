use crate::data::{self, ModuleId};
use crate::{Command, Error, ErrorKind, State};
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
        scope.push("commands", rhai::Array::default());
        scope.push_constant("scenario", scenario.clone());
        scope.push_constant("state", state.clone());

        let mut engine = rhai::Engine::new();
        engine.register_type::<data::Scenario>();
        engine.register_type::<State>();
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

        for module in modules {
            let name = required_modules.get(&module.id()).unwrap();
            engine.register_static_module(format!("battlefield::{name}"), module.ast().unwrap());
            engine.run_with_scope(
                &mut scope,
                &format!("commands = battlefield::{name}::commands(scenario, state, commands);"),
            )?;
        }
        scope.get_value::<rhai::Array>("commands").unwrap()
            .into_iter()
            .map(|string| Ok(Command(string.into_string()?)))
            .collect::<Result<Vec<Command>, &str>>()
            .map_err(|type_name| crate::Error::internal(crate::ErrorKind::RuntimeError, format!("`commands` must return an array of strings, but it contained a {type_name}.")))
    }

    pub fn perform(
        &self,
        command: Command,
        scenario: &data::Scenario,
        state: &State,
    ) -> crate::Result<State> {
        let mut scope = rhai::Scope::new();
        scope.push_constant("command", command.0);
        scope.push_constant("scenario", scenario.clone());
        scope.push_constant("state", state.clone());

        let mut engine = rhai::Engine::new();
        engine
            .register_type::<data::Scenario>()
            .register_type::<State>()
            .register_fn("set_data", State::set_data)
            .register_fn("get_data", State::get_data);
        let required_modules = scenario
            .modules()
            .map(|(name, config)| (config.id(), name))
            .collect::<HashMap<_, _>>();

        let modules = self
            .modules
            .iter()
            .rev()
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

        for module in modules {
            let name = required_modules.get(&module.id()).unwrap();
            engine.register_static_module(format!("battlefield::{name}"), module.ast().unwrap());
            let result = engine.eval_with_scope::<rhai::Dynamic>(
                &mut scope,
                &format!("battlefield::{name}::perform(scenario, state, command)"),
            )?;
            if let Some(new_state) = result.try_cast() {
                return Ok(new_state);
            }
        }
        Ok(state.clone())
    }
}
