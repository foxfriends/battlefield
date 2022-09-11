use crate::database;
use crate::directory::LookupExisting;
use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};
use crate::graphql::{Context, Json};
use battlefield_core::data::Scenario;
use battlefield_core::{Command, RuntimeContext, State};
use juniper::FieldResult;

mod games_connector;
pub use games_connector::GamesConnector;

mod live_game;
use live_game::LiveGame;

pub struct Game(pub database::Game);

#[juniper::graphql_object(context = Context)]
impl Game {
    fn id(&self) -> String {
        self.0.id.to_string()
    }

    fn commands(&self, context: &Context) -> FieldResult<Vec<Json<Command>>> {
        Ok(context
            .engine
            .commands(RuntimeContext::new(self.0.scenario.clone()), &self.0.state)?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    fn simulate(&self, context: &Context, command: Json<Command>) -> FieldResult<Json<State>> {
        Ok(Json(context.engine.perform(
            command.0,
            &self.0.scenario,
            &self.0.state,
        )?))
    }

    async fn scenario(&self) -> Json<Scenario> {
        // TODO: Scenario can likely be converted into an actual GraphQL object
        Json(self.0.scenario.clone())
    }

    async fn state(&self) -> Json<State> {
        Json(self.0.state.clone())
    }

    async fn live<'a>(&'a self, context: &'a Context) -> FieldResult<Option<LiveGame<'a>>> {
        let addr = context.directory.send(LookupExisting(self.0.id)).await?;
        let addr = match addr {
            None => return Ok(None),
            Some(addr) => addr,
        };
        Ok(Some(LiveGame {
            game: &self.0,
            addr,
        }))
    }
}

impl ConnectionNode for Game {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.id.to_string())
    }
}

connection_edge!(impl for Game as "GameEdge");
