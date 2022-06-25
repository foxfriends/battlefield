use super::Game;
use actix::prelude::*;
use battlefield_core::{Command, State};

#[derive(Message)]
#[rtype(result = "anyhow::Result<(State, Vec<Command>)>")]
pub struct GetState;

impl Handler<GetState> for Game {
    type Result = MessageResult<GetState>;

    fn handle(&mut self, GetState: GetState, _ctx: &mut Self::Context) -> Self::Result {
        match self.engine.commands(&self.game.scenario, &self.game.state) {
            Ok(actions) => MessageResult(Ok((self.game.state.clone(), actions))),
            Err(error) => MessageResult(Err(error.into())),
        }
    }
}
