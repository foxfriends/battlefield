use crate::data;
use crate::{Command, Error, ErrorKind};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod context;
mod state;

use context::{Context, CONTEXT_MODULE};
use state::{State, STATE_MODULE};

impl super::Engine {
    fn construct_engine<'a>(
        &self,
        scenario: &'a data::Scenario,
    ) -> crate::Result<(rhai::Engine, Vec<&'a str>)> {
        let mut engine = rhai::Engine::new();
        engine.register_global_module(CONTEXT_MODULE.clone());
        engine.register_global_module(STATE_MODULE.clone());

        let required_modules: HashMap<_, _> = scenario
            .modules()
            .map(|(name, config)| (config.id(), name))
            .collect();

        let modules: Vec<_> = self
            .modules
            .iter()
            .filter(|module| module.is_valid())
            .filter(|module| required_modules.contains_key(&module.id()))
            .collect();

        if required_modules.len() != modules.len() {
            let missing: Vec<_> = required_modules
                .into_keys()
                .filter(|key| modules.iter().any(|module| module.id() == *key))
                .map(|key| key.to_string())
                .collect();
            return Err(Error::internal(
                ErrorKind::ModuleNotFound,
                format!(
                    "{} required modules were not found: {}",
                    missing.len(),
                    missing.join(", ")
                ),
            ));
        }

        let mut module_names_ordered = Vec::with_capacity(modules.len());
        for module in modules {
            let name = *required_modules.get(&module.id()).unwrap();
            engine.register_static_module(format!("battlefield::{name}"), module.ast().unwrap());
            module_names_ordered.push(name);
        }

        Ok((engine, module_names_ordered))
    }

    pub fn commands(
        &self,
        scenario: &data::Scenario,
        state: &crate::State,
    ) -> crate::Result<Vec<Command>> {
        let mut scope = rhai::Scope::new();
        let context = Arc::new(Mutex::new(Context::new(scenario.clone())));
        let state = Arc::new(Mutex::new(State::new(state.clone())));
        scope.push_constant("state", state);
        scope.push_constant("context", context.clone());
        let (engine, modules) = self.construct_engine(scenario)?;
        for name in modules {
            context.lock().unwrap().set_current_module(name);
            engine.run_with_scope(
                &mut scope,
                &format!("battlefield::{name}::commands(context, state);"),
            )?;
        }
        std::mem::drop(scope);
        Ok(Arc::try_unwrap(context)
            .unwrap()
            .into_inner()
            .unwrap()
            .into_commands())
    }

    pub(super) fn runtime_init(
        &self,
        scenario: &data::Scenario,
        state: crate::State,
    ) -> crate::Result<crate::State> {
        let mut scope = rhai::Scope::new();
        let context = Arc::new(Mutex::new(Context::new(scenario.clone())));
        let state = Arc::new(Mutex::new(State::new(state)));
        scope.push_constant("state", state.clone());
        scope.push_constant("context", context.clone());
        let (engine, modules) = self.construct_engine(scenario)?;
        for name in modules {
            context.lock().unwrap().set_current_module(name);
            engine.run_with_scope(
                &mut scope,
                &format!("battlefield::{name}::init(context, state);"),
            )?;
        }
        std::mem::drop(scope);
        Ok(Arc::try_unwrap(state)
            .unwrap()
            .into_inner()
            .unwrap()
            .into_inner())
    }

    pub fn perform(
        &self,
        command: Command,
        scenario: &data::Scenario,
        state: &crate::State,
    ) -> crate::Result<crate::State> {
        let mut scope = rhai::Scope::new();
        let context = Arc::new(Mutex::new(Context::new(scenario.clone())));
        let state = Arc::new(Mutex::new(State::new(state.clone())));
        scope.push_constant("command", command.0);
        scope.push_constant("state", state.clone());
        scope.push_constant("context", context.clone());
        let (engine, modules) = self.construct_engine(scenario)?;
        for name in modules.into_iter().rev() {
            context.lock().unwrap().set_current_module(name);
            engine.run_with_scope(
                &mut scope,
                &format!("battlefield::{name}::perform(context, state, command)"),
            )?;
        }
        std::mem::drop(scope);
        Ok(Arc::try_unwrap(state)
            .unwrap()
            .into_inner()
            .unwrap()
            .into_inner())
    }
}
