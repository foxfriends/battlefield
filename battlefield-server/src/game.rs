use crate::db::PgPool;
use actix::prelude::*;
use battlefield_core::{process, Scenario, State};
use serde_json::Value;
use uuid::Uuid;

pub struct Game {
    id: Uuid,
    scenario: Scenario,
    state: State,
    db: PgPool,
}

impl Game {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub async fn new(db: PgPool) -> anyhow::Result<Self> {
        let mut conn = db.acquire().await?;
        let game =
            sqlx::query!("INSERT INTO games (id) values (default) RETURNING id, state, scenario")
                .fetch_one(&mut conn)
                .await?;
        let state = serde_json::from_value(game.state)?;
        let scenario = serde_json::from_value(game.scenario)?;
        Ok(Self {
            id: game.id,
            scenario,
            state,
            db,
        })
    }

    pub async fn load(id: Uuid, db: PgPool) -> anyhow::Result<Self> {
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
        })
    }
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<Value>")]
pub struct Command(pub battlefield_core::Command);

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

impl Handler<Command> for Game {
    type Result = MessageResult<Command>;

    fn handle(&mut self, Command(command): Command, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(process(command, &self.scenario, &mut self.state))
    }
}

#[derive(Message)]
#[rtype(result = "State")]
pub struct GetState;

impl Handler<GetState> for Game {
    type Result = MessageResult<GetState>;

    fn handle(&mut self, GetState: GetState, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.state.clone())
    }
}
