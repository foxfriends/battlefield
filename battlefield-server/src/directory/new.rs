use super::Directory;
use crate::game::Game;
use actix::prelude::*;
use battlefield_core::Scenario;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "anyhow::Result<(Uuid, Addr<Game>)>")]
pub struct New(pub Scenario);

impl Handler<New> for Directory {
    type Result = ResponseFuture<anyhow::Result<(Uuid, Addr<Game>)>>;

    fn handle(&mut self, New(scenario): New, _ctx: &mut Self::Context) -> Self::Result {
        let directory = self.clone();
        Box::pin(async move {
            let game = Game::new(scenario, directory.db.clone(), directory.engine.clone()).await?;
            let id = game.id();
            let mut games = directory.games.lock().await;
            let addr = game.start();
            games.insert(id, addr.downgrade());
            Ok((id, addr))
        })
    }
}
