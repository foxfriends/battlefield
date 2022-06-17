use crate::graphql::schema::connection::connection_edge;

mod scenarios_connection;
pub use scenarios_connection::ScenariosConnection;

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
}

connection_edge!(impl<'a> for Scenario<'a> as "ScenarioEdge");
