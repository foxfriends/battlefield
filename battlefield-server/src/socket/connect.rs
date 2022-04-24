use super::handler::SocketHandler;
use crate::directory::{Directory, Lookup};
use actix::prelude::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Params {
    game_id: Uuid,
}

pub async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<Params>,
    directory: web::Data<Addr<Directory>>,
) -> Result<HttpResponse, Error> {
    let game = directory
        .send(Lookup(path.game_id))
        .await
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game directory has disconnected: {error}"))
        })?
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game failed to be created: {error}"))
        })?;
    ws::start(SocketHandler::new(path.game_id, game), &req, stream)
}
