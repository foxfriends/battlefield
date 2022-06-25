use super::Directory;
use crate::game::Game;
use actix::prelude::*;
use actix::WeakAddr;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Option<Addr<Game>>")]
pub struct LookupExisting(pub Uuid);

impl Handler<LookupExisting> for Directory {
    type Result = ResponseFuture<Option<Addr<Game>>>;

    fn handle(&mut self, LookupExisting(id): LookupExisting, _ctx: &mut Self::Context) -> Self::Result {
        let directory = self.clone();
        Box::pin(async move {
            let games = directory.games.lock().await;
            games.get(&id).and_then(WeakAddr::upgrade)
        })
    }
}
