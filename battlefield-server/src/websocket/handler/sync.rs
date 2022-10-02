use super::{Notification, SocketHandler};
use crate::game::{GetCommands, GetState};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub(super) struct Sync;

impl Handler<Sync> for SocketHandler {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(&mut self, Sync: Sync, ctx: &mut Self::Context) -> Self::Result {
        let game = self.game.clone();
        let player = self.player_name.clone();
        let socket = ctx.address();
        Box::pin(async move {
            match player {
                Some(player) => {
                    let state = game.send(GetState).await?;
                    let commands = game
                        .send(GetCommands::for_player(player))
                        .await
                        .map_err(Notification::error)
                        .and_then(|commands| commands.map_err(Notification::error));
                    let notification = match commands {
                        Ok(commands) => Notification::sync(state, commands),
                        Err(error) => error,
                    };
                    socket.do_send(notification);
                }
                None => {
                    socket.do_send(Notification::error("Must identify before syncing"));
                }
            }
            Ok(())
        })
    }
}
