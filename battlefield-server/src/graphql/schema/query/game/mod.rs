use crate::database;
use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};
use crate::graphql::{Context, Json};
use battlefield_core::Command;
use juniper::FieldResult;

mod games_connector;
pub use games_connector::GamesConnector;

pub struct Game(pub database::Game);

#[juniper::graphql_object(context = Context)]
impl Game {
    fn id(&self) -> String {
        self.0.id.to_string()
    }

    fn commands(&self, context: &Context) -> FieldResult<Vec<Json<Command>>> {
        Ok(context
            .engine
            .commands(&self.0.scenario, &self.0.state)?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }
}

impl ConnectionNode for Game {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.id.to_string())
    }
}

connection_edge!(impl for Game as "GameEdge");
