use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

mod handler;

use handler::Handler;

pub(super) async fn connect(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Handler, &req, stream)
}
