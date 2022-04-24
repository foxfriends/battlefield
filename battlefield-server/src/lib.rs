use actix::prelude::*;
use actix_web::web::{self, ServiceConfig};

mod db;
mod directory;
mod game;
mod socket;

use db::PgPool;
use directory::Directory;

#[derive(Clone)]
pub struct BattlefieldServer {
    db: PgPool,
    directory: Addr<Directory>,
}

impl BattlefieldServer {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let db = db::connect(database_url).await?;
        let directory = Directory::new(db.clone()).start();
        Ok(Self { db, directory })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        config
            .app_data(web::Data::new(self.directory.clone()))
            .app_data(web::Data::new(self.db.clone()))
            .route("/ws/new", web::get().to(socket::create))
            .route("/ws/{game_id}", web::get().to(socket::connect));
    }
}
