use crate::module::{Module, ModuleId};
use crate::util::flatten::Flatten;
use crate::{Command, Scenario, State};
use serde_json::Value;
use std::collections::{hash_map, HashMap};
use std::path::PathBuf;

// TODO: make this a real thing
#[derive(Copy, Clone, Debug)]
pub struct ModuleNotFound;

#[derive(Default)]
pub struct Engine {
    modules_path: Vec<PathBuf>,
    modules: HashMap<ModuleId, Module>,
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

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_modules_path(&mut self, path: PathBuf) {
        self.modules_path.push(path);
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

    fn require_module(&self, name: &str, version: &str) -> anyhow::Result<&Module> {
        let id = ModuleId::new(name.to_owned(), version.to_owned());
        self.modules
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Module {id} not found"))
    }

    pub fn commands(&self, scenario: &Scenario, state: &State) -> anyhow::Result<Vec<Command>> {
        scenario
            .modules
            .iter()
            .map(|(name, config)| {
                let module = self.require_module(name, &config.version)?;
                Ok(module.commands(scenario, state))
            })
            .collect::<anyhow::Result<Flatten<Vec<Command>>>>()
            .map(|f| f.0)
    }

    pub fn perform(
        &self,
        _command: Command,
        _scenario: &Scenario,
        _state: &mut State,
    ) -> anyhow::Result<Value> {
        Ok(Value::default())
    }
}
