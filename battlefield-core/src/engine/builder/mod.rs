use crate::ScenarioError;

use super::{Engine, Module, ModuleId, Scenario};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct EngineBuilder {
    modules_path: Vec<PathBuf>,
    scenarios_path: Vec<PathBuf>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_modules(mut self, path: impl AsRef<Path>) -> Self {
        self.modules_path.push(path.as_ref().to_owned());
        self
    }

    pub fn add_scenarios(mut self, path: impl AsRef<Path>) -> Self {
        self.scenarios_path.push(path.as_ref().to_owned());
        self
    }

    pub fn build(self) -> Engine {
        let mut scenarios = load_scenarios(&self.scenarios_path);
        let modules = load_modules(&self.modules_path);
        for scenario in &mut scenarios {
            let missing_modules: Vec<_> = scenario
                .modules()
                .filter(|module_config| !modules.contains_key(&module_config.id()))
                .map(|module_config| module_config.id())
                .collect();
            log::warn!(
                "Missing {} required modules for scenario {} ({})",
                missing_modules.len(),
                scenario.name(),
                scenario.path().display(),
            );
            for missing_module in missing_modules {
                scenario.add_error(ScenarioError::RequiredModuleNotFound(missing_module));
            }
        }

        Engine { scenarios, modules }
    }
}

fn load_modules(modules_path: &[PathBuf]) -> HashMap<ModuleId, Module> {
    modules_path
        .iter()
        .filter_map(|directory| std::fs::read_dir(directory).ok())
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if !metadata.is_dir() {
                return None;
            }
            let name = entry.file_name().into_string().ok()?;
            let id = name.parse::<ModuleId>().ok()?;
            let module = Module::load(entry.path()).ok()?; // TODO: keep this error around too?
            Some((id, module))
        })
        .collect()
}

fn load_scenarios(scenarios_path: &[PathBuf]) -> Vec<Scenario> {
    scenarios_path
        .iter()
        .filter_map(|directory| std::fs::read_dir(directory).ok())
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if !metadata.is_file() {
                return None;
            }
            let path = entry.path();
            if path.extension().and_then(OsStr::to_str) != Some("toml") {
                return None;
            }
            Some(Scenario::from_file(path))
        })
        .collect()
}
