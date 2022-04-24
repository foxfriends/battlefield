use super::{Notification, SocketHandler};
use crate::game::GetState;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub(super) struct Sync;

impl Handler<Sync> for SocketHandler {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, Sync: Sync, ctx: &mut Self::Context) -> Self::Result {
        let game = self.game.clone();
        let socket = ctx.address();
        Box::pin(async move {
            let response = game
                .send(GetState)
                .await
                .map(|(state, actions)| Notification::Sync { state, actions })
                .unwrap_or_else(Notification::error);
            socket.do_send(response);
        })
    }
}
