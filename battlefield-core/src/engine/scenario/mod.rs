use crate::data::{self, ModuleId};
use std::path::{Path, PathBuf};

mod scenario_error;
pub use scenario_error::ScenarioError;

pub struct Scenario {
    path: PathBuf,
    data: Option<data::Scenario>,
    errors: Vec<ScenarioError>,
}

impl Scenario {
    pub(super) fn from_file(path: PathBuf) -> Self {
        let data = std::fs::read_to_string(&path)
            .and_then(|scenario_toml| toml::from_str(&scenario_toml).map_err(Into::into))
            .map_err(Into::into)
            .map_err(ScenarioError::FailedToLoad);

        let (data, errors) = match data {
            Ok(data) => (Some(data), vec![]),
            Err(error) => (None, vec![error]),
        };

        Scenario { path, data, errors }
    }

    pub(super) fn add_error(&mut self, error: ScenarioError) {
        self.errors.push(error);
    }

    pub fn is_valid(&self) -> bool {
        self.data.is_some() && self.errors.is_empty()
    }

    pub fn data(&self) -> Option<&data::Scenario> {
        self.data.as_ref()
    }

    pub fn name(&self) -> &str {
        self.data
            .as_ref()
            .map(|data| data.name())
            .unwrap_or_else(|| self.path.to_str().unwrap())
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn modules(&self) -> impl Iterator<Item = &data::ModuleConfig> {
        self.data
            .iter()
            .flat_map(|scenario| scenario.modules.values())
    }

    pub fn errors(&self) -> &[ScenarioError] {
        &self.errors
    }
}
