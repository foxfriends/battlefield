use super::Game;
use actix::prelude::*;
use battlefield_core::Command;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Vec<Command>>")]
pub struct GetCommands;

impl Handler<GetCommands> for Game {
    type Result = MessageResult<GetCommands>;

    fn handle(&mut self, GetCommands: GetCommands, _ctx: &mut Self::Context) -> Self::Result {
        match self.engine.commands(&self.game.scenario, &self.game.state) {
            Ok(actions) => MessageResult(Ok(actions)),
            Err(error) => MessageResult(Err(error.into())),
        }
    }
}
