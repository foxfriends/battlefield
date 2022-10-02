use super::Game;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct GetPlayers;

impl Handler<GetPlayers> for Game {
    type Result = MessageResult<GetPlayers>;

    fn handle(&mut self, GetPlayers: GetPlayers, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.game.players.clone())
    }
}
