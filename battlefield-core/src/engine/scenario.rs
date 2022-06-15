use super::module::ModuleId;
use crate::data;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScenarioError {
    RequiredModuleNotFound(ModuleId),
    FailedToLoadRequiredModule(ModuleId, Option<crate::Error>),
}

pub(crate) struct Scenario {
    path: PathBuf,
    data: crate::Result<data::Scenario>,
    errors: Vec<ScenarioError>,
}

impl Scenario {
    pub(super) fn from_file(path: PathBuf) -> Self {
        let data = std::fs::read_to_string(&path)
            .and_then(|scenario_toml| toml::from_str(&scenario_toml).map_err(Into::into))
            .map_err(Into::into);
        Scenario {
            path,
            data,
            errors: vec![],
        }
    }

    pub(super) fn add_error(&mut self, error: ScenarioError) {
        self.errors.push(error);
    }

    pub fn data(&self) -> Option<&data::Scenario> {
        self.data.as_ref().ok()
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn is(&self, scenario: &data::Scenario) -> bool {
        self.data
            .as_ref()
            .map(|data| std::ptr::eq(data, scenario))
            .unwrap_or(false)
    }

    pub fn modules(&self) -> impl Iterator<Item = &data::ModuleConfig> {
        self.data
            .iter()
            .flat_map(|scenario| scenario.modules.values())
    }
}
