use crate::game::{Game, GetScenario, GetState};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

mod command;
mod response;
mod sync;

use command::Command;
use response::Response;
use sync::Sync;

pub(super) struct SocketHandler {
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
        let future = async move {
            let state = game.send(GetState).await.unwrap();
            let scenario = game.send(GetScenario).await.unwrap();
            socket.do_send(Response::Ok(json! {{
                "id": game_id,
                "scenario": scenario,
                "state": state,
            }}));
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
                Err(error) => ctx.notify(Response::error(error)),
            },
            _ => {}
        }
    }
}
