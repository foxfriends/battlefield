use super::{Notification, SocketHandler};
use crate::game::{GetState, GetCommands};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub(super) struct Sync;

impl Handler<Sync> for SocketHandler {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(&mut self, Sync: Sync, ctx: &mut Self::Context) -> Self::Result {
        let game = self.game.clone();
        let socket = ctx.address();
        Box::pin(async move {
            let state = game.send(GetState).await?;
            let commands = game
                .send(GetCommands)
                .await
                .map_err(Notification::error)
                .and_then(|commands| commands.map_err(Notification::error));
            let notification = match commands {
                Ok(commands) => Notification::Sync { state, commands },
                Err(error) => error,
            };
            socket.do_send(notification);
            Ok(())
        })
    }
}
