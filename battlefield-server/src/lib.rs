use actix::prelude::*;
use actix_web::web::{self, ServiceConfig};
use battlefield_core::Engine;
use std::sync::Arc;

mod database;
mod directory;
mod game;
pub mod graphql;
mod websocket;

use database::PgPool;
use directory::Directory;

#[derive(Clone)]
pub struct BattlefieldServer {
    database: PgPool,
    engine: Arc<Engine>,
    directory: Addr<Directory>,
}

impl BattlefieldServer {
    pub async fn new(database_url: &str, engine: Engine) -> anyhow::Result<Self> {
        let database = database::connect(database_url).await?;
        let engine = Arc::new(engine);
        let directory = Directory::new(database.clone(), engine.clone()).start();

        Ok(Self {
            database,
            engine,
            directory,
        })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        config
            .app_data(web::Data::new(self.directory.clone()))
            .app_data(web::Data::new(self.database.clone()))
            .app_data(web::Data::from(self.engine.clone()))
            .service(graphql::service())
            .service(websocket::service());
    }
}
