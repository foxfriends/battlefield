use crate::game::{Game, Subscribe};
use actix::prelude::*;
use actix_web_actors::ws;
use battlefield_api::websocket as api;
use uuid::Uuid;

mod command;
mod identify;
mod notification;
mod sync;

pub(crate) use notification::Notification;

use command::Command;
use identify::Identify;
use sync::Sync;

pub struct SocketHandler {
    game_id: Uuid,
    player_name: Option<String>,
    game: Addr<Game>,
}

impl SocketHandler {
    pub fn new(game_id: Uuid, game: Addr<Game>) -> Self {
        Self {
            game_id,
            player_name: None,
            game,
        }
    }
}

impl Actor for SocketHandler {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::debug!("SocketHandler for {} started", self.game_id);
        let socket = ctx.address();
        self.game.do_send(Subscribe(socket.downgrade()));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::debug!("SocketHandler for {} stopped", self.game_id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => match serde_json::from_str(&text) {
                Ok(api::Message::Identify(name)) => ctx.notify(Identify(name)),
                Ok(api::Message::Sync) => ctx.notify(Sync),
                Ok(api::Message::Command(command)) => ctx.notify(Command(command.into())),
                Err(error) => ctx.notify(Notification::error(error)),
            },
            _ => {}
        }
    }
}
