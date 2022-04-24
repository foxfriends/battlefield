use super::{Response, SocketHandler};
use crate::game::GetState;
use actix::prelude::*;
use serde_json::json;

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
                .map(|state| json! {{ "state": state }})
                .map(Response::ok)
                .unwrap_or_else(Response::error);
            socket.do_send(response);
        })
    }
}
