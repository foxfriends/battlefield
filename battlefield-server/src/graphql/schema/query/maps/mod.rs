use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};

mod maps_connector;
pub use maps_connector::MapsConnector;

pub struct Map<'a>(pub(super) &'a battlefield_core::Map);

#[juniper::graphql_object]
impl Map<'_> {
    fn path(&self) -> String {
        self.0.path().display().to_string()
    }

    fn name(&self) -> Option<&str> {
        self.0.name()
    }

    fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    fn errors(&self) -> Vec<String> {
        self.0.errors().iter().map(ToString::to_string).collect()
    }
}

impl ConnectionNode for Map<'_> {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.path().display().to_string())
    }
}

connection_edge!(impl<'a> for Map<'a> as "ModuleEdge");
