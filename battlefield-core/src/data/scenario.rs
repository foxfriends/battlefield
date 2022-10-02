use super::{module_map, ModuleConfig, Player};
use battlefield_api as api;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Scenario {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) map: String,
    #[serde(default)]
    pub(crate) players: Vec<Player>,
    #[serde(deserialize_with = "module_map")]
    pub(crate) modules: HashMap<String, ModuleConfig>,
}

impl Scenario {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn modules(&self) -> impl Iterator<Item = (&str, &ModuleConfig)> {
        self.modules
            .iter()
            .map(|(name, config)| (name.as_str(), config))
    }

    pub fn module(&self, module: &str) -> Option<&ModuleConfig> {
        self.modules.get(module)
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn with_players(self, players: Vec<String>) -> Self {
        Self {
            players: players
                .into_iter()
                .enumerate()
                .map(|(id, name)| Player::new(id, name))
                .collect(),
            ..self
        }
    }

    pub fn expected_player_count(&self) -> Option<usize> {
        self.modules
            .values()
            .find(|module| module.name() == "core")?
            .config
            .get("players")?
            .as_integer()
            .map(|int| int as usize)
    }
}

impl From<Scenario> for api::Scenario {
    fn from(scenario: Scenario) -> Self {
        Self {
            name: scenario.name,
            description: scenario.description,
            map: scenario.map,
            players: scenario
                .players
                .into_iter()
                .map(|player| player.into())
                .collect(),
            modules: scenario
                .modules
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}
