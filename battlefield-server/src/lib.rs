use actix::prelude::*;
use actix_web::web::{self, ServiceConfig};
use battlefield_core::Engine;
use std::path::Path;

mod db;
mod directory;
mod game;
mod scenarios;
mod socket;

use db::PgPool;
use directory::Directory;
use scenarios::Scenarios;

#[derive(Clone)]
pub struct BattlefieldServer {
    db: PgPool,
    directory: Addr<Directory>,
    scenarios: Scenarios,
}

impl BattlefieldServer {
    pub async fn new(
        database_url: &str,
        scenarios_dir: &Path,
        engine: Engine,
    ) -> anyhow::Result<Self> {
        let db = db::connect(database_url).await?;
        let directory = Directory::new(db.clone(), engine).start();
        let scenarios = Scenarios::new(scenarios_dir.to_owned());
        Ok(Self {
            db,
            directory,
            scenarios,
        })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        config
            .app_data(web::Data::new(self.directory.clone()))
            .app_data(web::Data::new(self.db.clone()))
            .app_data(web::Data::new(self.scenarios.clone()))
            .route("/ws/new/{scenario}", web::get().to(socket::create))
            .route("/ws/{game_id}", web::get().to(socket::connect));
    }
}
