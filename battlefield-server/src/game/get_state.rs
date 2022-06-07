use super::Game;
use actix::prelude::*;
use battlefield_core::{Command, State};

#[derive(Message)]
#[rtype(result = "anyhow::Result<(State, Vec<Command>)>")]
pub struct GetState;

impl Handler<GetState> for Game {
    type Result = ResponseFuture<anyhow::Result<(State, Vec<Command>)>>;

    fn handle(&mut self, GetState: GetState, _ctx: &mut Self::Context) -> Self::Result {
        let scenario = self.scenario.clone();
        let state = self.state.clone();
        let engine = self.engine.clone();

        Box::pin(async move {
            let engine = engine.read().await;
            let actions = engine.commands(&scenario, &state)?;
            Ok((state, actions))
        })
    }
}
