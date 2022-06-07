use crate::db::PgPool;
use crate::socket::SocketHandler;
use actix::prelude::*;
use actix::WeakAddr;
use battlefield_core::{Engine, Scenario, State};
use std::sync::Arc;
use uuid::Uuid;

mod command;
mod commit;
mod get_scenario;
mod get_state;
mod subscribe;

pub use command::Command;
use commit::Commit;
pub use get_scenario::GetScenario;
pub use get_state::GetState;
pub use subscribe::Subscribe;

pub struct Game {
    id: Uuid,
    scenario: Scenario,
    state: State,
    db: PgPool,
    subscribers: Vec<WeakAddr<SocketHandler>>,
    engine: Arc<Engine>,
}

impl Game {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub async fn new(scenario: Scenario, db: PgPool, engine: Arc<Engine>) -> anyhow::Result<Self> {
        let mut conn = db.acquire().await?;
        let scenario_json = serde_json::to_value(&scenario).unwrap();
        let game = sqlx::query!(
            "INSERT INTO games (scenario) values ($1) RETURNING id, state",
            scenario_json
        )
        .fetch_one(&mut conn)
        .await?;
        let state = serde_json::from_value(game.state)?;
        Ok(Self {
            id: game.id,
            scenario,
            state,
            db,
            subscribers: vec![],
            engine,
        })
    }

    pub async fn load(id: Uuid, db: PgPool, engine: Arc<Engine>) -> anyhow::Result<Self> {
        let mut conn = db.acquire().await?;
        let game = sqlx::query!("SELECT state, scenario FROM games WHERE id = $1", id)
            .fetch_one(&mut conn)
            .await?;
        let state = serde_json::from_value(game.state)?;
        let scenario = serde_json::from_value(game.scenario)?;
        Ok(Self {
            id,
            state,
            scenario,
            db,
            subscribers: vec![],
            engine,
        })
    }
}

impl Actor for Game {
    type Context = Context<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::debug!("Stopping and saving game {}", self.id);
        let db = self.db.clone();
        let json_state = serde_json::to_value(&self.state).unwrap();
        let id = self.id;
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
