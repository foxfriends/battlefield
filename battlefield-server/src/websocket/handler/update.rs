use super::{Notification, SocketHandler};
use crate::game::GetCommands;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub(crate) struct Update(pub json_patch::Patch);

impl Handler<Update> for SocketHandler {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(&mut self, Update(patch): Update, ctx: &mut Self::Context) -> Self::Result {
        let game = self.game.clone();
        let player = self.player_name.clone().unwrap();
        let socket = ctx.address();
        Box::pin(async move {
            let commands = game
                .send(GetCommands::for_player(player))
                .await
                .map_err(Notification::error)
                .and_then(|commands| commands.map_err(Notification::error));
            let notification = match commands {
                Ok(commands) => Notification::update(patch, commands),
                Err(error) => error,
            };
            socket.do_send(notification);
            Ok(())
        })
    }
}
