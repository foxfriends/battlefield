use crate::{Command, Scenario, State};
use json_patch::Patch;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    Init {
        id: Uuid,
        scenario: Scenario,
        state: State,
        commands: Vec<Command>,
    },
    Sync {
        state: State,
        commands: Vec<Command>,
    },
    Update {
        patch: Patch,
        commands: Vec<Command>,
    },
    Err(String),
}
