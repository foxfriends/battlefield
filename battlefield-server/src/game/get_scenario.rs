use super::Game;
use actix::prelude::*;
use battlefield_core::data::Scenario;

#[derive(Message)]
#[rtype(result = "Scenario")]
pub struct GetScenario;

impl Handler<GetScenario> for Game {
    type Result = MessageResult<GetScenario>;

    fn handle(&mut self, GetScenario: GetScenario, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.game.scenario.clone())
    }
}
