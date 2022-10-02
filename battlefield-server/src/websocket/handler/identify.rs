use super::{Notification, SocketHandler};
use crate::game::{GetCommands, GetPlayers, GetScenario, GetState};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub(super) struct Identify(pub String);

impl Handler<Identify> for SocketHandler {
    type Result = ();

    fn handle(&mut self, Identify(name): Identify, ctx: &mut Self::Context) {
        self.player_name = Some(name.clone());

        let game = self.game.clone();
        let game_id = self.game_id;
        let socket = ctx.address();
        let future = async move {
            let state = match game.send(GetState).await {
                Ok(state) => state,
                Err(error) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
            };
            let commands = match game.send(GetCommands::for_player(name)).await {
                Ok(Ok(commands)) => commands,
                Ok(Err(error)) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
                Err(error) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
            };
            let players = match game.send(GetPlayers).await {
                Ok(players) => players,
                Err(error) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
            };
            let scenario = match game.send(GetScenario).await {
                Ok(scenario) => scenario,
                Err(error) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
            };
            socket.do_send(Notification::init(
                game_id, scenario, state, players, commands,
            ));
        };
        future.into_actor(self).spawn(ctx);
    }
}
