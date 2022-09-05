use battlefield_api::{Command, Scenario, State};
use json_patch::{Patch, PatchError};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub commands: Vec<Command>,
    pub state: State,
    scenario: Rc<Scenario>,
}

impl GameState {
    pub(super) fn new(commands: Vec<Command>, state: State, scenario: Scenario) -> Self {
        Self {
            commands,
            state,
            scenario: Rc::new(scenario),
        }
    }

    pub(super) fn sync(&self, commands: Vec<Command>, state: State) -> Self {
        Self {
            commands,
            state,
            scenario: self.scenario.clone(),
        }
    }

    pub(super) fn update(&self, commands: Vec<Command>, patch: &Patch) -> Result<Self, PatchError> {
        let mut state_json = serde_json::to_value(&self.state).unwrap();
        json_patch::patch(&mut state_json, patch)?;
        let state = serde_json::from_value(state_json).unwrap();
        Ok(Self {
            commands,
            state,
            scenario: self.scenario.clone(),
        })
    }

    #[allow(dead_code)]
    pub fn scenario(&self) -> &Scenario {
        self.scenario.as_ref()
    }
}
