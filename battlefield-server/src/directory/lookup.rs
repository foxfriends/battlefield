use super::Directory;
use crate::game::Game;
use actix::prelude::*;
use std::collections::hash_map::Entry;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Addr<Game>>")]
pub struct Lookup(pub Uuid);

impl Handler<Lookup> for Directory {
    type Result = ResponseFuture<anyhow::Result<Addr<Game>>>;

    fn handle(&mut self, Lookup(id): Lookup, _ctx: &mut Self::Context) -> Self::Result {
        let directory = self.clone();
        Box::pin(async move {
            let mut games = directory.games.lock().await;
            match games.entry(id) {
                Entry::Occupied(mut entry) => match entry.get().upgrade() {
                    Some(addr) => Ok(addr),
                    None => {
                        let game =
                            Game::load(id, directory.db.clone(), directory.engine.clone()).await?;
                        let addr = game.start();
                        entry.insert(addr.downgrade());
                        Ok(addr)
                    }
                },
                Entry::Vacant(entry) => {
                    let game =
                        Game::load(id, directory.db.clone(), directory.engine.clone()).await?;
                    let addr = game.start();
                    entry.insert(addr.downgrade());
                    Ok(addr)
                }
            }
        })
    }
}
