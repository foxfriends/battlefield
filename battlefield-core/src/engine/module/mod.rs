use crate::data::{ModuleConfig, ModuleId, ModuleManifest};
use rhai::module_resolvers::{FileModuleResolver, ModuleResolversCollection, StaticModuleResolver};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

mod module_error;

pub use module_error::ModuleError;

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
        modules: &[Module],
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
        let mut static_modules = StaticModuleResolver::new();
        for (name, config) in manifest.dependencies() {
            let id = config.id();
            let module = modules
                .iter()
                .find(|module| module.id() == id)
                .and_then(|module| module.ast());
            match module {
                Some(module) => {
                    // TODO: see if the Rhai guys agree to allow StaticModuleResolver to accept a `Shared<Module>`
                    static_modules.insert(name, (*module).clone());
                }
                None => errors.push(ModuleError::UnresolvedDependency(id)),
            }
        }
        let mut resolver = ModuleResolversCollection::new();
        resolver.push(FileModuleResolver::new_with_path(&path));
        resolver.push(static_modules);
        engine.set_module_resolver(resolver);

        let ast = errors
            .is_empty()
            .then(|| {
                let ast = read_to_string(path.join(manifest.entrypoint()))
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

    pub fn dependencies(&self) -> impl Iterator<Item = &ModuleConfig> {
        self.manifest
            .iter()
            .flat_map(ModuleManifest::dependencies)
            .map(|(_, config)| config)
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

    pub fn is_valid(&self) -> bool {
        self.manifest.is_some() && self.ast.is_some() && self.errors.is_empty()
    }

    pub fn errors(&self) -> &[ModuleError] {
        &self.errors
    }
}
