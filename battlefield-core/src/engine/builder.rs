use super::Engine;
use crate::module::{Module, ModuleId};
use crate::Scenario;
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
        for scenario_result in &mut scenarios {
            if let Ok(scenario) = scenario_result {
                for (name, config) in &scenario.modules {
                    if let Err(error) = modules.resolve_module(name, &config.version) {
                        *scenario_result = Err(error);
                        break;
                    }
                }
            }
        }

        Engine {
            // TODO: maybe keep those things around as Results so they can be reported as errors?
            scenarios: scenarios.into_iter().filter_map(Result::ok).collect(),
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

    fn resolve_module(&mut self, name: &str, version: &str) -> anyhow::Result<&Module> {
        let entry = self
            .modules
            .entry(ModuleId::new(name.to_owned(), version.to_owned()));

        match entry {
            hash_map::Entry::Occupied(entry) => Ok(entry.into_mut()),
            hash_map::Entry::Vacant(entry) => {
                let module = load_module(&self.modules_path, entry.key())?;
                Ok(entry.insert(module))
            }
        }
    }
}

fn load_module(modules_path: &[PathBuf], id: &ModuleId) -> anyhow::Result<Module> {
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
        .ok_or_else(|| anyhow::anyhow!("Module {id} not found"))?;
    Module::load(module_path)
}

fn load_scenarios(scenarios_path: &[PathBuf]) -> Vec<anyhow::Result<Scenario>> {
    scenarios_path
        .iter()
        .filter_map(|directory| std::fs::read_dir(directory).ok())
        .flatten()
        .map(|entry| {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if !metadata.is_file() {
                return Ok(None);
            }
            let path = entry.path();
            if path.extension().and_then(OsStr::to_str) != Some("toml") {
                return Ok(None);
            }
            let scenario_toml = std::fs::read_to_string(path)?;
            Ok(toml::from_str(&scenario_toml)?)
        })
        .filter_map(Result::transpose)
        .collect()
}
