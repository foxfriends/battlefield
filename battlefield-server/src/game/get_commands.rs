use super::Game;
use actix::prelude::*;
use battlefield_core::{Command, RuntimeContext};

#[derive(Message)]
#[rtype(result = "anyhow::Result<Vec<Command>>")]
pub struct GetCommands;

impl Handler<GetCommands> for Game {
    type Result = MessageResult<GetCommands>;

    fn handle(&mut self, GetCommands: GetCommands, _ctx: &mut Self::Context) -> Self::Result {
        let context = RuntimeContext::new(self.game.scenario.clone());
        match self.engine.commands(context, &self.game.state) {
            Ok(actions) => MessageResult(Ok(actions)),
            Err(error) => MessageResult(Err(error.into())),
        }
    }
}
