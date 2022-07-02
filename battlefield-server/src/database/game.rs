use battlefield_core::data::Scenario;
use battlefield_core::State;
use sqlx::{Executor, Postgres};
use uuid::Uuid;

pub struct RawGame {
    pub id: Uuid,
    pub scenario: serde_json::Value,
    pub state: serde_json::Value,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub id: Uuid,
    pub scenario: Scenario,
    pub state: State,
}

impl Game {
    pub async fn create(
        scenario: Scenario,
        state: State,
        conn: impl Executor<'_, Database = Postgres>,
    ) -> anyhow::Result<Self> {
        let scenario_json = serde_json::to_value(&scenario)?;
        let state_json = serde_json::to_value(&state)?;
        let data = sqlx::query!(
            "INSERT INTO games (scenario, state) values ($1, $2) RETURNING id",
            scenario_json,
            state_json,
        )
        .fetch_one(conn)
        .await?;
        Ok(Self {
            id: data.id,
            scenario,
            state,
        })
    }

    pub async fn load(
        id: Uuid,
        conn: impl Executor<'_, Database = Postgres>,
    ) -> anyhow::Result<Self> {
        sqlx::query_as!(RawGame, "SELECT * FROM games WHERE id = $1", id)
            .fetch_one(conn)
            .await?
            .try_into()
    }
}

impl TryFrom<RawGame> for Game {
    type Error = anyhow::Error;

    fn try_from(raw: RawGame) -> Result<Self, Self::Error> {
        let state = serde_json::from_value(raw.state)?;
        let scenario = serde_json::from_value(raw.scenario)?;
        Ok(Self {
            id: raw.id,
            state,
            scenario,
        })
    }
}
