use super::Scenario;
use crate::{data::ModuleManifest, Command, State};
use rhai::module_resolvers::FileModuleResolver;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

mod module_id;

pub use module_id::ModuleId;

pub enum ModuleError {
    ManifestError(crate::Error),
    SourceError(crate::Error),
    UnresolvedDependency(ModuleId),
}

pub struct Module {
    name: String,
    version: String,
    path: PathBuf,
    manifest: Option<ModuleManifest>,
    ast: Option<rhai::Shared<rhai::Module>>,
    errors: Vec<ModuleError>,
}

impl Module {
    pub(crate) fn load(
        path: PathBuf,
        manifest: crate::Result<ModuleManifest>,
        modules: &HashMap<ModuleId, Module>,
    ) -> Self {
        // NOTE: these are all unwrapping because this method should only be called
        // after the caller has already validated the path. That may have to change
        // someday.
        let mut segments = path.file_name().unwrap().to_str().unwrap().split('@');
        let name = segments.next().unwrap();
        let version = segments.next().unwrap();

        let manifest = match manifest {
            Ok(manifest) => manifest,
            Err(error) => {
                return Self {
                    name: name.to_owned(),
                    version: version.to_owned(),
                    path,
                    manifest: None,
                    ast: None,
                    errors: vec![ModuleError::ManifestError(error)],
                }
            }
        };

        let mut errors = vec![];
        let mut engine = rhai::Engine::new();
        for (name, config) in manifest.dependencies() {
            let id = config.id();
            match modules.get(&id).and_then(|module| module.ast()) {
                Some(module) => {
                    engine.register_static_module(format!("battlefield::{name}"), module);
                }
                None => errors.push(ModuleError::UnresolvedDependency(id)),
            }
        }
        engine.set_module_resolver(FileModuleResolver::new_with_path(&path));

        let ast = errors
            .is_empty()
            .then(|| {
                let ast = read_to_string(path.join(&manifest.entrypoint))
                    .map_err(Into::into)
                    .and_then(|src| {
                        engine
                            .compile_into_self_contained(&rhai::Scope::default(), &src)
                            .map_err(Into::into)
                    })
                    .and_then(|ast| {
                        rhai::Module::eval_ast_as_new(rhai::Scope::default(), &ast, &engine)
                            .map_err(Into::into)
                    })
                    .map(rhai::Shared::new);
                match ast {
                    Ok(ast) => Some(ast),
                    Err(error) => {
                        errors.push(ModuleError::SourceError(error));
                        None
                    }
                }
            })
            .flatten();

        Self {
            name: name.to_owned(),
            version: version.to_owned(),
            path,
            manifest: Some(manifest),
            ast,
            errors,
        }
    }

    pub(crate) fn ast(&self) -> Option<rhai::Shared<rhai::Module>> {
        self.ast.clone()
    }

    pub(crate) fn commands(
        &self,
        scenario: &Scenario,
        state: &State,
    ) -> crate::Result<Vec<Command>> {
        Ok(vec![])
    }

    #[allow(dead_code)]
    pub(crate) fn perform(
        &self,
        _command: Command,
        _scenario: &Scenario,
        _state: &mut State,
    ) -> crate::Result<Value> {
        Ok(Value::default())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn id(&self) -> ModuleId {
        ModuleId::new(self.name.clone(), self.version.clone())
    }
}
