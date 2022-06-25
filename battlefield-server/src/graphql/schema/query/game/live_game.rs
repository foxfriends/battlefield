use crate::database;
use crate::game::{Game, GetCommands};
use crate::graphql::{Context, Json};
use actix::Addr;
use battlefield_core::Command;
use juniper::FieldResult;

pub struct LiveGame<'a> {
    pub(super) game: &'a database::Game,
    pub(super) addr: Addr<Game>,
}

#[juniper::graphql_object(context = Context)]
impl<'a> LiveGame<'a> {
    fn id(&self) -> String {
        self.game.id.to_string()
    }

    async fn commands(&self) -> FieldResult<Vec<Json<Command>>> {
        let commands = self.addr.send(GetCommands).await??;
        Ok(commands.into_iter().map(Json).collect())
    }
}
