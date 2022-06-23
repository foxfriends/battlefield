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
pub struct ModuleMetadata {
    pub(crate) name: String,
    pub(crate) version: String,
    #[serde(default = "default_entrypoint")]
    pub(crate) entrypoint: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleManifest {
    pub(crate) module: ModuleMetadata,
    #[serde(deserialize_with = "module_map", default)]
    pub(crate) dependencies: HashMap<String, ModuleConfig>,
}

impl ModuleManifest {
    pub fn entrypoint(&self) -> &Path {
        &self.module.entrypoint
    }

    pub fn name(&self) -> &str {
        &self.module.name
    }

    pub fn version(&self) -> &str {
        &self.module.version
    }

    pub fn dependencies(&self) -> impl Iterator<Item = (&str, &ModuleConfig)> {
        self.dependencies
            .iter()
            .map(|(name, config)| (name.as_str(), config))
    }
}
