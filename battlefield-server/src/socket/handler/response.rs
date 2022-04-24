use super::SocketHandler;
use actix::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub(super) enum Response {
    Ok(Value),
    Err(Value),
}

impl Response {
    pub fn ok(value: impl Serialize) -> Self {
        Response::Ok(serde_json::to_value(value).unwrap())
    }

    pub fn error(err: impl std::fmt::Display) -> Self {
        Response::Err(Value::String(err.to_string()))
    }
}

impl Handler<Response> for SocketHandler {
    type Result = ();

    fn handle(&mut self, response: Response, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}
