use super::Game;
use actix::prelude::*;
use battlefield_core::{commands, Command, State};

#[derive(Message)]
#[rtype(result = "(State, Vec<Command>)")]
pub struct GetState;

impl Handler<GetState> for Game {
    type Result = MessageResult<GetState>;

    fn handle(&mut self, GetState: GetState, _ctx: &mut Self::Context) -> Self::Result {
        let actions = commands(&self.scenario, &self.state);
        MessageResult((self.state.clone(), actions))
    }
}
