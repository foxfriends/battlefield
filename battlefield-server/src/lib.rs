use actix::prelude::*;
use actix_web::web::{self, ServiceConfig};
use battlefield_core::Engine;
use std::sync::Arc;

mod db;
mod directory;
mod game;
mod graphql;
mod websocket;

use db::PgPool;
use directory::Directory;

#[derive(Clone)]
pub struct BattlefieldServer {
    db: PgPool,
    engine: Arc<Engine>,
    directory: Addr<Directory>,
}

impl BattlefieldServer {
    pub async fn new(database_url: &str, engine: Engine) -> anyhow::Result<Self> {
        let db = db::connect(database_url).await?;
        let engine = Arc::new(engine);
        let directory = Directory::new(db.clone(), engine.clone()).start();

        Ok(Self {
            db,
            engine,
            directory,
        })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        config
            .app_data(web::Data::new(self.directory.clone()))
            .app_data(web::Data::new(self.db.clone()))
            .app_data(web::Data::from(self.engine.clone()))
            .configure(graphql::configure)
            .configure(websocket::configure);
    }
}
