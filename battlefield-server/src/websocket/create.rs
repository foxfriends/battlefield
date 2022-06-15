use std::sync::Arc;

use super::handler::SocketHandler;
use crate::directory::{Directory, New};
use actix::prelude::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use battlefield_core::Engine;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    scenario: String,
}

pub async fn create(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<Params>,
    engine: web::Data<Arc<Engine>>,
    directory: web::Data<Addr<Directory>>,
) -> Result<HttpResponse, Error> {
    let scenario = engine.scenario(&path.scenario).ok_or_else(move || {
        error::ErrorNotFound(format!("Scenario {} not found", path.scenario))
    })?;
    let (game_id, game) = directory
        .send(New(scenario.clone()))
        .await
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game directory has disconnected: {error}"))
        })?
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game failed to be created: {error}"))
        })?;
    ws::start(SocketHandler::new(game_id, game), &req, stream)
}
