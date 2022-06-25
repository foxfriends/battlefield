use crate::game::{Game, GetCommands, GetScenario, GetState, Subscribe};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

mod command;
mod notification;
mod sync;

pub use notification::Notification;

use command::Command;
use sync::Sync;

pub struct SocketHandler {
    game_id: Uuid,
    game: Addr<Game>,
}

impl SocketHandler {
    pub fn new(game_id: Uuid, game: Addr<Game>) -> Self {
        Self { game_id, game }
    }
}

impl Actor for SocketHandler {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::debug!("SocketHandler for {} started", self.game_id);
        let game = self.game.clone();
        let game_id = self.game_id;
        let socket = ctx.address();
        game.do_send(Subscribe(socket.downgrade()));
        let future = async move {
            let state = match game.send(GetState).await {
                Ok(state) => state,
                Err(error) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
            };
            let commands = match game.send(GetCommands).await {
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
            let scenario = match game.send(GetScenario).await {
                Ok(scenario) => scenario,
                Err(error) => {
                    socket.do_send(Notification::error(error));
                    return;
                }
            };
            socket.do_send(Notification::Init {
                id: game_id,
                scenario,
                state,
                commands,
            });
        };
        future.into_actor(self).spawn(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::debug!("SocketHandler for {} stopped", self.game_id);
    }
}

#[derive(Deserialize)]
enum SocketMessage {
    Sync,
    Command(battlefield_core::Command),
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => match serde_json::from_str(&text) {
                Ok(SocketMessage::Sync) => ctx.notify(Sync),
                Ok(SocketMessage::Command(command)) => ctx.notify(Command(command)),
                Err(error) => ctx.notify(Notification::error(error)),
            },
            _ => {}
        }
    }
}
