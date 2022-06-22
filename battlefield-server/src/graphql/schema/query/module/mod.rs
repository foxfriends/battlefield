use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};

mod modules_connector;
pub use modules_connector::ModulesConnector;

pub struct Module<'a>(pub(super) &'a battlefield_core::Module);

#[juniper::graphql_object]
impl Module<'_> {
    fn id(&self) -> String {
        self.0.id().to_string()
    }

    fn path(&self) -> String {
        self.0.path().display().to_string()
    }

    fn name(&self) -> &str {
        self.0.name()
    }

    fn version(&self) -> &str {
        self.0.version()
    }

    fn dependencies(&self) -> Vec<String> {
        self.0
            .dependencies()
            .map(|module| module.id().to_string())
            .collect()
    }

    fn errors(&self) -> Vec<String> {
        self.0.errors().map(ToString::to_string).collect()
    }
}

impl ConnectionNode for Module<'_> {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.id().to_string())
    }
}

connection_edge!(impl<'a> for Module<'a> as "ModuleEdge");
