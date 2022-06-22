use crate::data::ModuleId;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ModuleError {
    ManifestError(crate::Error),
    SourceError(crate::Error),
    UnresolvedDependency(ModuleId),
}

impl std::error::Error for ModuleError {}

impl Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ManifestError(error) => {
                write!(f, "Failed to parse manifest: {error}")
            }
            Self::SourceError(error) => {
                write!(f, "Failed to parse module: {error}")
            }
            Self::UnresolvedDependency(module_id) => {
                write!(f, "Unresolved dependency `{module_id}`")
            }
        }
    }
}
