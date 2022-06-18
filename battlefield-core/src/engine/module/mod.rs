use super::Scenario;
use crate::{Command, State};
use serde_json::Value;
use std::path::{Path, PathBuf};

mod module_id;

pub use module_id::ModuleId;

pub struct Module {
    name: String,
    version: String,
    path: PathBuf,
}

impl Module {
    pub(crate) fn load(path: PathBuf) -> crate::Result<Self> {
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

    pub(crate) fn commands(&self, _scenario: &Scenario, _state: &State) -> Vec<Command> {
        vec![]
    }

    pub(crate) fn perform(
        &self,
        _command: Command,
        _scenario: &Scenario,
        _state: &mut State,
    ) -> crate::Result<Value> {
        Ok(Value::default())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn id(&self) -> ModuleId {
        ModuleId::new(self.name.clone(), self.version.clone())
    }
}
