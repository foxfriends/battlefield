use crate::database;
use crate::game::{Game, GetCommands, GetScenario, GetState};
use crate::graphql::Json;
use actix::Addr;
use battlefield_core::data::Scenario;
use battlefield_core::{Command, State};
use juniper::FieldResult;

pub struct LiveGame<'a> {
    pub(super) game: &'a database::Game,
    pub(super) addr: Addr<Game>,
}

#[juniper::graphql_object]
impl<'a> LiveGame<'a> {
    fn id(&self) -> String {
        self.game.id.to_string()
    }

    async fn scenario(&self) -> FieldResult<Json<Scenario>> {
        // TODO: Scenario can likely be converted into an actual GraphQL object
        Ok(Json(self.addr.send(GetScenario).await?))
    }

    async fn state(&self) -> FieldResult<Json<State>> {
        Ok(Json(self.addr.send(GetState).await?))
    }

    async fn commands(&self, player: String) -> FieldResult<Vec<Json<Command>>> {
        let commands = self.addr.send(GetCommands::for_player(player)).await??;
        Ok(commands.into_iter().map(Json).collect())
    }
}
