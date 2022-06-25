use super::{Commit, Game};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Command(pub battlefield_core::Command);

impl Handler<Command> for Game {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(&mut self, Command(command): Command, ctx: &mut Self::Context) -> Self::Result {
        let state = self.game.state.clone();
        let scenario = self.game.scenario.clone();
        let engine = self.engine.clone();
        let addr = ctx.address();
        Box::pin(async move {
            let new_state = engine.perform(command, &scenario, &state)?;
            addr.send(Commit(new_state)).await??;
            Ok(())
        })
    }
}
