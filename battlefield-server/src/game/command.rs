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
        MessageResult(process(command, &self.scenario, &mut self.state))
    }
}
