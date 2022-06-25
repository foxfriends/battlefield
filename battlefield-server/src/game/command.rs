use super::{Commit, Game};
use actix::prelude::*;
use serde_json::Value;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Value>")]
pub struct Command(pub battlefield_core::Command);

impl Handler<Command> for Game {
    type Result = ResponseFuture<anyhow::Result<Value>>;

    fn handle(&mut self, Command(command): Command, ctx: &mut Self::Context) -> Self::Result {
        let mut state = self.game.state.clone();
        let scenario = self.game.scenario.clone();
        let engine = self.engine.clone();
        let addr = ctx.address();
        Box::pin(async move {
            let response = engine.perform(command, &scenario, &mut state)?;
            addr.send(Commit(state)).await??;
            Ok(response)
        })
    }
}
