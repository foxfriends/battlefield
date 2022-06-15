<<<<<<< Updated upstream
use actix_web::web;
use actix_web::dev::Payload;
=======
use actix_web::dev::Payload;
use actix_web::web;
>>>>>>> Stashed changes
use actix_web::FromRequest;
use actix_web::HttpRequest;
use battlefield_core::Engine;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct Context {
    pub(super) engine: Arc<Engine>,
}

impl FromRequest for Context {
    type Error = <web::Data<Engine> as FromRequest>::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let engine = web::Data::<Engine>::from_request(req, payload);
        Box::pin(async move {
            let engine = engine.await?.into_inner();
            Ok(Self { engine })
        })
    }
}
