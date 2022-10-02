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

macro_rules! try_authorized {
    ($a:expr) => {
        match $a {
            Some(auth) => auth,
            None => return Err("Unauthorized".into()),
        }
    };
}

#[juniper::graphql_object(context = Context)]
impl Game {
    fn id(&self) -> String {
        self.0.id.to_string()
    }

    fn commands(
        &self,
        context: &Context,
        player: Option<String>,
    ) -> FieldResult<Vec<Json<Command>>> {
        let player = try_authorized!(player.or_else(|| context.player.clone()));
        Ok(context
            .engine
            .commands(
                RuntimeContext::new(self.0.scenario.clone(), Some(player)),
                &self.0.state,
            )?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    fn simulate(
        &self,
        context: &Context,
        command: Json<Command>,
        player: Option<String>,
    ) -> FieldResult<Json<State>> {
        let player = try_authorized!(player.or_else(|| context.player.clone()));
        Ok(Json(context.engine.perform(
            command.0,
            RuntimeContext::new(self.0.scenario.clone(), Some(player)),
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
