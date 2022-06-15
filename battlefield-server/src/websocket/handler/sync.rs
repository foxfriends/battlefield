use super::{Notification, SocketHandler};
use crate::game::GetState;
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
            let response = game
                .send(GetState)
                .await
                .map(|result| match result {
                    Ok((state, actions)) => Notification::Sync { state, actions },
                    Err(error) => Notification::error(error),
                })
                .unwrap_or_else(Notification::error);
            socket.do_send(response);
            Ok(())
        })
    }
}
