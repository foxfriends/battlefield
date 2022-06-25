use crate::database::PgPool;
use crate::directory::Directory;
use actix::Addr;
use actix_web::dev::Payload;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use battlefield_core::Engine;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct Context {
    pub(super) engine: Arc<Engine>,
    // In the graphql situation, we'll be prone to multiple connections per request.
    // Some sort of sub-pooling of a single connection might be nice?
    pub(super) database: PgPool,
    pub(super) directory: Addr<Directory>,
}

impl juniper::Context for Context {}

impl FromRequest for Context {
    type Error = <web::Data<Engine> as FromRequest>::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let engine = web::Data::<Engine>::from_request(req, payload);
        let database = web::Data::<PgPool>::from_request(req, payload);
        let directory = web::Data::<Addr<Directory>>::from_request(req, payload);
        Box::pin(async move {
            let engine = engine.await?.into_inner();
            let database = (**database.await?).clone();
            let directory = (**directory.await?).clone();
            Ok(Self { engine, database, directory })
        })
    }
}
