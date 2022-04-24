use crate::game::{Command, Game};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

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

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::debug!("SocketHandler for {} started", self.game_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::debug!("SocketHandler for {} stopped", self.game_id);
    }
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub enum Response {
    Ok(Value),
    Err(Value),
}

impl Response {
    fn error(err: impl std::fmt::Display) -> Self {
        Response::Err(Value::String(err.to_string()))
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let command = match text.parse() {
                    Ok(command) => command,
                    Err(error) => {
                        ctx.notify(Response::error(error));
                        return;
                    }
                };
                let game = self.game.clone();
                let socket = ctx.address();
                let future = async move {
                    let response = game
                        .send(Command(command))
                        .await
                        .map_err(anyhow::Error::from)
                        .and_then(|result| result)
                        .map(Response::Ok)
                        .unwrap_or_else(Response::error);
                    socket.do_send(response);
                };
                future.into_actor(self).spawn(ctx);
            }
            _ => {}
        }
    }
}

impl Handler<Response> for SocketHandler {
    type Result = ();

    fn handle(&mut self, response: Response, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}
