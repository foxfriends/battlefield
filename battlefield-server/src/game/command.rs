use super::{Commit, Game};
use actix::prelude::*;
use battlefield_core::RuntimeContext;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Command {
    command: battlefield_core::Command,
    player: String,
}

impl Command {
    pub fn new(command: battlefield_core::Command, player: String) -> Self {
        Self { command, player }
    }
}

impl Handler<Command> for Game {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(
        &mut self,
        Command { command, player }: Command,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let state = self.game.state.clone();
        let engine = self.engine.clone();
        let context = RuntimeContext::new(self.game.scenario.clone(), Some(player));
        let addr = ctx.address();
        Box::pin(async move {
            let new_state = engine.perform(command, context, &state)?;
            addr.send(Commit(new_state)).await??;
            Ok(())
        })
    }
}
