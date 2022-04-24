use super::Game;
use actix::prelude::*;
use battlefield_core::State;

#[derive(Message)]
#[rtype(result = "State")]
pub struct GetState;

impl Handler<GetState> for Game {
    type Result = MessageResult<GetState>;

    fn handle(&mut self, GetState: GetState, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.state.clone())
    }
}
