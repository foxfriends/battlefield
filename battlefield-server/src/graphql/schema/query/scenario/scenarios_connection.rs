use super::Scenario;
use crate::graphql::schema::connection::{Connection, ConnectionNode, Cursor, Edge, PageInfo};

pub struct ScenariosConnection<'a> {
    pub(super) edges: Vec<Edge<Scenario<'a>>>,
    pub(super) page_info: PageInfo,
}

impl<'a> Connection for ScenariosConnection<'a> {
    type Node = Scenario<'a>;

    fn edges(&self) -> &[Edge<Self::Node>] {
        &self.edges
    }

    fn page_info(&self) -> PageInfo {
        self.page_info.clone()
    }
}

impl ConnectionNode for Scenario<'_> {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.name().to_owned())
    }
}
