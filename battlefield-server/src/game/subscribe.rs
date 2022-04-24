use super::Game;
use crate::socket::SocketHandler;
use actix::prelude::*;
use actix::WeakAddr;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe(pub WeakAddr<SocketHandler>);

impl Handler<Subscribe> for Game {
    type Result = ();

    fn handle(&mut self, Subscribe(addr): Subscribe, _ctx: &mut Self::Context) {
        self.subscribers.push(addr);
    }
}
