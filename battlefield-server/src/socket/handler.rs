use crate::directory::{Directory, Lookup};
use crate::game::Command;
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

pub(super) struct SocketHandler {
    pub game_id: Uuid,
    pub directory: Addr<Directory>,
}

impl Actor for SocketHandler {
    type Context = ws::WebsocketContext<Self>;
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
                match text.parse() {
                    Ok(command) => {
                        let directory = self.directory.clone();
                        let id = self.game_id;
                        let socket = ctx.address();
                        let future = async move {
                            let game = directory
                                .send(Lookup(id))
                                .await
                                .map_err(anyhow::Error::from)
                                .and_then(|result| result);
                            let game = match game {
                                Ok(game) => game,
                                Err(error) => {
                                    socket.send(Response::error(error)).await.ok();
                                    return;
                                }
                            };
                            let value = game
                                .send(Command(command))
                                .await
                                .map_err(anyhow::Error::from)
                                .and_then(|result| result);
                            let value = match value {
                                Ok(value) => value,
                                Err(error) => {
                                    socket.send(Response::error(error)).await.ok();
                                    return;
                                }
                            };
                            socket.send(Response::Ok(value)).await.ok();
                        };
                        future.into_actor(self).spawn(ctx);
                    }
                    Err(error) => {
                        ctx.notify(Response::error(error));
                    }
                };
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
