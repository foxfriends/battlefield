use super::ModuleId;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ScenarioError {
    MissingMap(String),
    FailedToLoad(crate::Error),
    RequiredModuleNotFound(ModuleId),
}

impl std::error::Error for ScenarioError {}

impl Display for ScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FailedToLoad(error) => {
                write!(f, "Failed to load: {error}")
            }
            Self::RequiredModuleNotFound(module_id) => {
                write!(f, "Required module {module_id} not found")
            }
            Self::MissingMap(map) => {
                write!(f, "Map {map} not found")
            }
        }
    }
}
