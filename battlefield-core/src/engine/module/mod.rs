use super::Scenario;
use crate::{data::ModuleManifest, Command, State};
use rhai::module_resolvers::FileModuleResolver;
use serde_json::Value;
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
    ast: Option<rhai::AST>,
    errors: Vec<ModuleError>,
}

impl Module {
    pub(crate) fn load(path: PathBuf, manifest: crate::Result<ModuleManifest>) -> Self {
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

        let mut engine = rhai::Engine::new();
        engine.set_module_resolver(FileModuleResolver::new_with_path(&path));

        let ast = read_to_string(path.join(&manifest.entrypoint))
            .map_err(Into::into)
            .and_then(|src| {
                engine
                    .compile_into_self_contained(&rhai::Scope::default(), &src)
                    .map_err(Into::into)
            });

        let (ast, errors) = match ast {
            Ok(ast) => (Some(ast), vec![]),
            Err(error) => (None, vec![ModuleError::SourceError(error)]),
        };

        Self {
            name: name.to_owned(),
            version: version.to_owned(),
            path,
            manifest: Some(manifest),
            ast,
            errors,
        }
    }

    pub(crate) fn ast(&self) -> Option<&rhai::AST> {
        self.ast.as_ref()
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
