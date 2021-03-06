use super::{Engine, Map, Module, ModuleId, Scenario};
use crate::data::ModuleManifest;
use crate::ScenarioError;
use log::Level;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct EngineBuilder {
    modules_path: Vec<PathBuf>,
    maps_path: Vec<PathBuf>,
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

    pub fn add_maps(mut self, path: impl AsRef<Path>) -> Self {
        self.maps_path.push(path.as_ref().to_owned());
        self
    }

    pub fn add_scenarios(mut self, path: impl AsRef<Path>) -> Self {
        self.scenarios_path.push(path.as_ref().to_owned());
        self
    }

    pub fn build(self) -> Engine {
        let maps = load_maps(&self.maps_path);
        let modules = load_modules(&self.modules_path);
        let scenarios = load_scenarios(&self.scenarios_path, &modules, &maps);
        Engine {
            maps,
            scenarios,
            modules,
        }
    }
}

fn load_modules(modules_path: &[PathBuf]) -> Vec<Module> {
    // load each module's manifest
    let mut manifests: HashMap<ModuleId, (PathBuf, crate::Result<ModuleManifest>)> = modules_path
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
            let id = name.parse().ok()?;
            // Errors above aren't really errors, just files that don't qualify as modules.
            // Errors below are errors with loading what should be a module.
            let path = entry.path();
            let manifest = std::fs::read_to_string(path.join("module.toml"))
                .map_err(Into::into)
                .and_then(|src| toml::from_str(&src).map_err(Into::into));
            Some((id, (path, manifest)))
        })
        .collect();

    log::info!("{} installed modules detected.", manifests.len());

    // Compute dependency order (Topological sort)
    // TODO: For all the rest that cannot be sorted... shove them in at the
    // end and they will encounter errors.
    let ids: Vec<_> = manifests.keys().cloned().collect();
    let edges: Vec<(ModuleId, ModuleId)> = manifests
        .iter()
        .flat_map(|(id, (_, manifest))| {
            manifest
                .iter()
                .flat_map(ModuleManifest::dependencies)
                .map(|(_, config)| config.id())
                .map(|dependency| (dependency, id.clone()))
        })
        .collect::<Vec<_>>();
    let mut incoming = edges.iter().fold(
        HashMap::<&ModuleId, HashSet<&ModuleId>>::with_capacity(manifests.len()),
        |mut edges, (from, to)| {
            edges.entry(to).or_default().insert(from);
            edges
        },
    );
    for id in &ids {
        incoming.entry(id).or_default();
    }
    let outgoing = edges.iter().fold(
        HashMap::<&ModuleId, HashSet<&ModuleId>>::with_capacity(manifests.len()),
        |mut edges, (from, to)| {
            edges.entry(from).or_default().insert(to);
            edges
        },
    );
    let mut sorted: Vec<&ModuleId> = Vec::with_capacity(manifests.len());
    let mut unlocked: Vec<&ModuleId> = incoming
        .iter()
        .filter(|(_, from_set)| from_set.is_empty())
        .map(|(to, _)| *to)
        .collect();
    incoming.retain(|_, set| !set.is_empty());
    while !unlocked.is_empty() {
        let source = unlocked.pop().unwrap();
        sorted.push(source);
        for destination in outgoing.get(source).iter().copied().flat_map(HashSet::iter) {
            if let Some(set) = incoming.get_mut(destination) {
                set.remove(source);
                if set.is_empty() {
                    unlocked.push(destination);
                    incoming.remove(destination);
                }
            }
        }
    }

    log::debug!(
        "Dependency graph: {:?}",
        sorted
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(" -> ")
    );
    for (id, _) in incoming.into_iter() {
        sorted.push(id);
    }

    // Load all modules for real
    let mut modules = Vec::with_capacity(manifests.len());
    for id in sorted.into_iter() {
        // If a dependency is not installed, we just ignore it
        // The dependent module will detect it is missing and report the error.
        if let Some((path, manifest)) = manifests.remove(id) {
            let module = Module::load(path, manifest, &modules);
            if log::log_enabled!(Level::Warn) && !module.is_valid() {
                log::warn!(
                    "Module {} failed to load with {} errors.",
                    module.id(),
                    module.errors().len()
                );
                if log::log_enabled!(Level::Debug) {
                    for error in module.errors().iter() {
                        log::debug!("{}", error)
                    }
                }
            }
            modules.push(module);
        }
    }

    if log::log_enabled!(Level::Info) {
        log::info!(
            "{} installed modules loaded successfully.",
            modules.iter().filter(|module| module.is_valid()).count()
        );
    }

    modules
}

fn load_maps(maps_path: &[PathBuf]) -> Vec<Map> {
    let maps: Vec<Map> = maps_path
        .iter()
        .filter_map(|directory| std::fs::read_dir(directory).ok())
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if !metadata.is_dir() {
                return None;
            }
            let map = Map::from_file(entry.path());
            Some(map)
        })
        .collect();
    log::info!("{} installed maps detected.", maps.len());
    log::info!(
        "{} installed maps loaded successfully.",
        maps.iter().filter(|map| map.is_valid()).count()
    );
    maps
}

fn load_scenarios(scenarios_path: &[PathBuf], modules: &[Module], maps: &[Map]) -> Vec<Scenario> {
    let mut scenarios: Vec<_> = scenarios_path
        .iter()
        .filter_map(|directory| std::fs::read_dir(directory).ok())
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if !metadata.is_dir() {
                return None;
            }
            let path = entry.path();
            Some(Scenario::from_file(path.join("scenario.toml")))
        })
        .collect();

    log::info!("{} installed scenarios detected.", scenarios.len());
    for scenario in scenarios
        .iter_mut()
        .filter(|scenario| scenario.data().is_some())
    {
        let data = scenario.data().unwrap();
        let map = maps
            .iter()
            .filter_map(|map| map.name())
            .find(|name| *name == data.map);
        if map.is_none() {
            scenario.add_error(ScenarioError::MissingMap(data.map.clone()));
        }

        let found_modules: HashSet<_> = scenario
            .modules()
            .filter_map(|module_config| {
                modules
                    .iter()
                    .find(|module| module.id() == module_config.id())
            })
            .filter(|module| module.is_valid())
            .map(|module| module.id())
            .collect();
        let missing_modules: Vec<_> = scenario
            .modules()
            .map(|module| module.id())
            .filter(|id| !found_modules.contains(id))
            .collect();
        if !missing_modules.is_empty() {
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
    }

    if log::log_enabled!(Level::Debug) {
        log::info!(
            "{} installed scenarios loaded successfully.",
            scenarios
                .iter()
                .filter(|scenario| scenario.is_valid())
                .count()
        );
    }

    scenarios
}
