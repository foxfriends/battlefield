use super::Game;
use actix::prelude::*;
use battlefield_core::process;
use serde_json::Value;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Value>")]
pub struct Command(pub battlefield_core::Command);

impl Handler<Command> for Game {
    type Result = MessageResult<Command>;

    fn handle(&mut self, Command(command): Command, _ctx: &mut Self::Context) -> Self::Result {
        let mut state = self.state.clone();
        let response = match process(command, &self.scenario, &mut state) {
            Ok(response) => response,
            Err(error) => return MessageResult(Err(error)),
        };
        self.commit(state);
        MessageResult(Ok(response))
    }
}
