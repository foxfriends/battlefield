use super::scenario::ScenarioError;
use super::{Engine, Module, ModuleId, Scenario};
use std::collections::{hash_map, HashMap};
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
        let mut modules = Modules::new(self.modules_path);
        for scenario in &mut scenarios {
            let errors: Vec<_> = scenario
                .modules()
                .filter_map(|module_config| modules.resolve_module(module_config.id()).err())
                .collect();
            log::warn!(
                "Encountered {} errors loading scenario from {}",
                errors.len(),
                scenario.path().display(),
            );
            for error in errors {
                log::debug!(
                    "Error loading scenario from {}: {:?}",
                    scenario.path().display(),
                    error,
                );
                scenario.add_error(error);
            }
        }

        Engine {
            scenarios,
            modules: modules.modules,
        }
    }
}

struct Modules {
    modules_path: Vec<PathBuf>,
    modules: HashMap<ModuleId, Module>,
}

impl Modules {
    fn new(modules_path: Vec<PathBuf>) -> Self {
        Self {
            modules_path,
            modules: HashMap::default(),
        }
    }

    fn resolve_module(&mut self, id: ModuleId) -> Result<&Module, ScenarioError> {
        let entry = self.modules.entry(id);

        match entry {
            hash_map::Entry::Occupied(entry) => Ok(entry.into_mut()),
            hash_map::Entry::Vacant(entry) => {
                let module = load_module(&self.modules_path, entry.key())?;
                Ok(entry.insert(module))
            }
        }
    }
}

fn load_module(modules_path: &[PathBuf], id: &ModuleId) -> Result<Module, ScenarioError> {
    let module_path = modules_path
        .iter()
        .filter_map(|directory| std::fs::read_dir(directory).ok())
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            if !metadata.is_dir() {
                return None;
            }
            let name = entry.file_name().into_string().ok()?;
            if name.parse::<ModuleId>().ok()? != *id {
                return None;
            }
            Some(entry.path())
        })
        .next()
        .ok_or_else(|| ScenarioError::RequiredModuleNotFound(id.clone()))?;
    match Module::load(module_path) {
        Ok(module) => Ok(module),
        Err(error) => Err(ScenarioError::FailedToLoadRequiredModule(
            id.clone(),
            Some(error),
        )),
    }
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
