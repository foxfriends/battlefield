#![allow(dead_code)]
use super::Player;
use crate::data::Scenario;
use crate::Command;
use rhai::plugin::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Context {
    scenario: Scenario,
    players: Vec<Player>,
    current_player: Option<usize>,
    current_module: String,
    commands: Vec<Command>,
}

impl Context {
    pub fn new(scenario: Scenario, _player: Option<String>) -> Self {
        Self {
            scenario,
            players: vec![],
            current_player: None, // TODO: we also need the list of players
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

    pub(super) fn scenario(&self) -> &Scenario {
        &self.scenario
    }
}

lazy_static::lazy_static! {
    pub(crate) static ref CONTEXT_MODULE: rhai::Shared<rhai::Module> = rhai::Shared::new(rhai::exported_module!(plugin_context));
}

#[allow(clippy::mut_mutex_lock)]
#[export_module]
mod plugin_context {
    use crate::util::toml_to_rhai::toml_to_rhai;

    pub type Context = Arc<Mutex<super::Context>>;
    pub type Config = crate::data::ModuleConfig;

    #[rhai_fn(get = "players", pure)]
    pub fn get_players(context: &mut Context) -> Vec<Player> {
        context.lock().unwrap().players.clone()
    }

    #[rhai_fn(get = "current_player", pure)]
    pub fn get_current_player(context: &mut Context) -> Dynamic {
        let context = context.lock().unwrap();
        if let Some(id) = context.current_player {
            Dynamic::from(context.players[id].clone())
        } else {
            Dynamic::UNIT
        }
    }

    #[rhai_fn(get = "module", pure)]
    pub fn get_module(context: &mut Context) -> String {
        context.lock().unwrap().current_module.clone()
    }

    #[rhai_fn(get = "config", pure)]
    pub fn get_config(context: &mut Context) -> Config {
        let context = context.lock().unwrap();
        let config = context.scenario.module(&context.current_module).unwrap();
        config.clone()
    }

    #[rhai_fn(index_get)]
    pub fn get_config_value(config: &mut Config, index: &str) -> Dynamic {
        config
            .config
            .get(index)
            .map(toml_to_rhai)
            .unwrap_or_else(|| Dynamic::from(()))
    }
}
