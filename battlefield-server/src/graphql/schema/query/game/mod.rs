use crate::database;
use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};

mod games_connector;
pub use games_connector::GamesConnector;

pub struct Game(pub database::Game);

#[juniper::graphql_object]
impl Game {
    fn id(&self) -> String {
        self.0.id.to_string()
    }
}

impl ConnectionNode for Game {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.id.to_string())
    }
}

connection_edge!(impl for Game as "GameEdge");
