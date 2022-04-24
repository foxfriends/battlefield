use super::{Response, SocketHandler};
use crate::game;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub(super) struct Command(pub battlefield_core::Command);

impl Handler<Command> for SocketHandler {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, Command(command): Command, ctx: &mut Self::Context) -> Self::Result {
        let game = self.game.clone();
        let socket = ctx.address();
        Box::pin(async move {
            let response = game
                .send(game::Command(command))
                .await
                .map_err(anyhow::Error::from)
                .and_then(|result| result)
                .map(Response::Ok)
                .unwrap_or_else(Response::error);
            socket.do_send(response);
        })
    }
}
