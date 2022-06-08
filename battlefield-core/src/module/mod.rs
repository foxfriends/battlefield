#![allow(dead_code)]

use crate::{Command, Scenario, State};
use serde_json::Value;
use std::path::PathBuf;

mod module_id;

pub use module_id::ModuleId;

pub(crate) struct Module {
    name: String,
    version: String,
    path: PathBuf,
}

impl Module {
    pub fn load(path: PathBuf) -> crate::Result<Self> {
        // NOTE: these are all unwrapping because this method should only be called
        // after the caller has already validated the path. That may have to change
        // someday
        let mut segments = path.file_name().unwrap().to_str().unwrap().split('@');
        let name = segments.next().unwrap();
        let version = segments.next().unwrap();
        // BUT: since it's all unwrapped, why is this a Result?
        Ok(Self {
            name: name.to_owned(),
            version: version.to_owned(),
            path,
        })
    }

    pub fn commands(&self, _scenario: &Scenario, _state: &State) -> Vec<Command> {
        vec![]
    }

    pub fn perform(
        &self,
        _command: Command,
        _scenario: &Scenario,
        _state: &mut State,
    ) -> crate::Result<Value> {
        Ok(Value::default())
    }
}
