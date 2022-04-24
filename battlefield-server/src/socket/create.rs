use super::handler::SocketHandler;
use crate::directory::{Directory, New};
use crate::scenarios::Scenarios;
use actix::prelude::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    scenario: String,
}

pub async fn create(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<Params>,
    scenarios: web::Data<Scenarios>,
    directory: web::Data<Addr<Directory>>,
) -> Result<HttpResponse, Error> {
    let scenario = scenarios.load(&path.scenario).await.map_err(move |error| {
        error::ErrorNotFound(format!("Scenario {} not found: {}", path.scenario, error))
    })?;
    let (game_id, game) = directory
        .send(New(scenario))
        .await
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game directory has disconnected: {error}"))
        })?
        .map_err(|error| {
            error::ErrorInternalServerError(format!("Game failed to be created: {error}"))
        })?;
    ws::start(SocketHandler::new(game_id, game), &req, stream)
}
