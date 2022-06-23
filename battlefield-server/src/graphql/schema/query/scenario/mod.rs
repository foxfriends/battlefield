use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};

mod scenarios_connector;
pub use scenarios_connector::ScenariosConnector;

pub struct Scenario<'a>(pub(super) &'a battlefield_core::Scenario);

#[juniper::graphql_object]
impl Scenario<'_> {
    fn path(&self) -> String {
        self.0.path().display().to_string()
    }

    fn name(&self) -> &str {
        self.0.name()
    }

    fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    fn description(&self) -> Option<&str> {
        self.0.data().map(|data| data.description())
    }

    fn errors(&self) -> Vec<String> {
        self.0.errors().iter().map(ToString::to_string).collect()
    }
}

impl ConnectionNode for Scenario<'_> {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.name().to_owned())
    }
}

connection_edge!(impl<'a> for Scenario<'a> as "ScenarioEdge");
