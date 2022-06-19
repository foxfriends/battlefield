use super::{module_map, ModuleConfig};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn default_entrypoint() -> PathBuf {
    PathBuf::from("lib.rhai")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleManifest {
    #[serde(default = "default_entrypoint")]
    pub(crate) entrypoint: PathBuf,
    #[serde(deserialize_with = "module_map")]
    pub(crate) dependencies: HashMap<String, ModuleConfig>,
}

impl ModuleManifest {
    pub fn entrypoint(&self) -> &Path {
        &self.entrypoint
    }

    pub fn dependencies(&self) -> impl Iterator<Item = (&str, &ModuleConfig)> {
        self.dependencies
            .iter()
            .map(|(name, config)| (name.as_str(), config))
    }
}
