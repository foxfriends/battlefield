use crate::database::{self, PgPool};
use crate::websocket::SocketHandler;
use actix::prelude::*;
use actix::WeakAddr;
use battlefield_core::{data::Scenario, Engine};
use std::sync::Arc;
use uuid::Uuid;

mod command;
mod commit;
mod get_commands;
mod get_scenario;
mod get_state;
mod subscribe;

pub use command::Command;
use commit::Commit;
pub use get_commands::GetCommands;
pub use get_scenario::GetScenario;
pub use get_state::GetState;
pub use subscribe::Subscribe;

pub struct Game {
    game: database::Game,
    db: PgPool,
    subscribers: Vec<WeakAddr<SocketHandler>>,
    engine: Arc<Engine>,
}

impl Game {
    pub fn id(&self) -> Uuid {
        self.game.id
    }

    pub async fn new(scenario: Scenario, db: PgPool, engine: Arc<Engine>) -> anyhow::Result<Self> {
        let mut conn = db.acquire().await?;
        let state = engine.initialize(&scenario);
        let game = database::Game::create(scenario, state, &mut conn).await?;
        Ok(Self {
            game,
            db,
            subscribers: vec![],
            engine,
        })
    }

    pub async fn load(id: Uuid, db: PgPool, engine: Arc<Engine>) -> anyhow::Result<Self> {
        let mut conn = db.acquire().await?;
        let game = database::Game::load(id, &mut conn).await?;
        Ok(Self {
            game,
            db,
            subscribers: vec![],
            engine,
        })
    }
}

impl Actor for Game {
    type Context = Context<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::debug!("Stopping and saving game {}", self.game.id);
        let db = self.db.clone();
        let json_state = serde_json::to_value(&self.game.state).unwrap();
        let id = self.game.id;
        let future = async move {
            match db.acquire().await {
                Ok(mut conn) => {
                    let result = sqlx::query!(
                        "UPDATE games SET state = $1 WHERE id = $2",
                        json_state,
                        id as Uuid
                    )
                    .execute(&mut conn)
                    .await
                    .map_err(anyhow::Error::from);
                    if let Err(error) = result {
                        log::error!("Failed to save game {id}: {error}");
                    }
                }
                Err(error) => {
                    log::error!("Failed to save game {id}: {error}");
                }
            }
        };
        future.into_actor(self).spawn(ctx);
    }
}
