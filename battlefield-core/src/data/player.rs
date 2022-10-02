use battlefield_api as api;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub(crate) id: usize,
    pub(crate) name: String,
}

impl Player {
    pub(super) fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<Player> for api::Player {
    fn from(player: Player) -> Self {
        Self {
            id: player.id,
            name: player.name,
        }
    }
}
