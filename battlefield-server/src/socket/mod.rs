use crate::directory::{Directory, New};
use actix::prelude::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

mod handler;

use handler::SocketHandler;

#[derive(Deserialize)]
pub struct Params {
    game_id: Uuid,
}

pub(super) async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<Params>,
    directory: web::Data<Addr<Directory>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        SocketHandler {
            game_id: path.game_id,
            directory: (**directory).clone(),
        },
        &req,
        stream,
    )
}

pub(super) async fn create(
    req: HttpRequest,
    stream: web::Payload,
    directory: web::Data<Addr<Directory>>,
) -> Result<HttpResponse, Error> {
    let game_id = directory
        .send(New)
        .await
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game directory has disconnected: {error}"))
        })?
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game failed to be created: {error}"))
        })?;
    ws::start(
        SocketHandler {
            game_id,
            directory: (**directory).clone(),
        },
        &req,
        stream,
    )
}
