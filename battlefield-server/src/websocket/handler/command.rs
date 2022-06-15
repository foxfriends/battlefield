use super::{Notification, SocketHandler};
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
            let result = game
                .send(game::Command(command))
                .await
                .map_err(anyhow::Error::from)
                .and_then(|result| result);
            if let Err(error) = result {
                socket.do_send(Notification::error(error));
            }
        })
    }
}
