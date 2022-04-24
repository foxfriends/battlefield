use crate::db::PgPool;
use crate::game::Game;
use actix::prelude::*;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct Directory {
    games: Arc<Mutex<HashMap<Uuid, Addr<Game>>>>,
    db: PgPool,
}

impl Directory {
    pub fn new(db: PgPool) -> Self {
        Self {
            games: Default::default(),
            db,
        }
    }
}

impl Actor for Directory {
    type Context = Context<Self>;
}

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
                Entry::Occupied(entry) => Ok(entry.get().clone()),
                Entry::Vacant(entry) => {
                    let game = Game::load(id, directory.db.clone()).await?;
                    let addr = game.start();
                    entry.insert(addr.clone());
                    Ok(addr)
                }
            }
        })
    }
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<(Uuid, Addr<Game>)>")]
pub struct New;

impl Handler<New> for Directory {
    type Result = ResponseFuture<anyhow::Result<(Uuid, Addr<Game>)>>;

    fn handle(&mut self, New: New, _ctx: &mut Self::Context) -> Self::Result {
        let directory = self.clone();
        Box::pin(async move {
            let game = Game::new(directory.db.clone()).await?;
            let id = game.id();
            let mut games = directory.games.lock().await;
            let addr = game.start();
            games.insert(id, addr.clone());
            Ok((id, addr))
        })
    }
}
