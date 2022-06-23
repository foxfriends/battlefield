mod util;

mod command;
pub mod data;
mod engine;
mod error;
mod state;

pub use command::Command;
pub use engine::{Engine, EngineBuilder, Module, Scenario, ScenarioError};
pub use error::{Error, ErrorKind, Result};
pub use state::State;

mod runtime {
    use lazy_static::lazy_static;
    use rhai::module_resolvers::DummyModuleResolver;
    use rhai::{Engine, Module, Shared};

    const RUNTIME_SOURCE: &'static str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/runtime/runtime.rhai"));

    lazy_static! {
        pub static ref MODULE: Shared<Module> = {
            let mut engine = Engine::new();
            engine.set_module_resolver(DummyModuleResolver);
            let ast = engine
                .compile_into_self_contained(&rhai::Scope::default(), RUNTIME_SOURCE)
                .unwrap();
            let module = Module::eval_ast_as_new(rhai::Scope::default(), &ast, &engine).unwrap();
            Shared::new(module)
        };
    }
}
