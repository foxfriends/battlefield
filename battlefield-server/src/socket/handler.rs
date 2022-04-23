use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use battlefield_core::{process, State};
use serde_json::json;

pub(super) struct Handler;

impl Actor for Handler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Handler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let message = text.parse().unwrap();
                let mut state = State::default();
                match process(message, &mut state) {
                    Ok(response) => ctx.text(
                        serde_json::to_string(&json! {{
                            "ok": response
                        }})
                        .unwrap(),
                    ),
                    Err(error) => {
                        ctx.text(
                            serde_json::to_string(&json! {{
                                "error": error.to_string(),
                            }})
                            .unwrap(),
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
