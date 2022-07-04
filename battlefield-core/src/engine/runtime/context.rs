#![allow(dead_code)]
use crate::data::Scenario;
use crate::Command;
use rhai::plugin::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Context {
    scenario: Scenario,
    current_module: String,
    commands: Vec<Command>,
}

impl Context {
    pub(super) fn new(scenario: Scenario) -> Self {
        Self {
            scenario: scenario,
            current_module: "*".to_owned(),
            commands: vec![],
        }
    }

    pub(super) fn set_current_module(&mut self, current_module: &str) {
        self.current_module = current_module.to_owned();
    }

    pub(super) fn into_commands(self) -> Vec<Command> {
        self.commands
    }
}

lazy_static::lazy_static! {
    pub(crate) static ref CONTEXT_MODULE: rhai::Shared<rhai::Module> = rhai::Shared::new(rhai::exported_module!(plugin_context));
}

#[export_module]
mod plugin_context {
    pub type Context = Arc<Mutex<super::Context>>;
}
