use super::Player;
use battlefield_core::data::Scenario;
use battlefield_core::State;
use sqlx::{Executor, Postgres};
use uuid::Uuid;

struct RawGame {
    id: Uuid,
    scenario: serde_json::Value,
    state: serde_json::Value,
    players: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub id: Uuid,
    pub scenario: Scenario,
    pub state: State,
    pub players: Vec<String>,
}

impl Game {
    pub async fn create<C>(
        scenario: Scenario,
        players: Vec<String>,
        state: State,
        conn: &mut C,
    ) -> anyhow::Result<Self>
    where
        for<'a> &'a mut C: Executor<'a, Database = Postgres>,
    {
        let scenario_json = serde_json::to_value(&scenario)?;
        let state_json = serde_json::to_value(&state)?;

        let data = sqlx::query!(
            "INSERT INTO games (scenario, state) VALUES ($1, $2) RETURNING id",
            scenario_json,
            state_json,
        )
        .fetch_one(&mut *conn)
        .await?;

        for player in &players {
            let player = Player::find_or_create(player, &mut *conn).await?;
            sqlx::query!(
                "INSERT INTO game_players (game_id, player_id) VALUES ($1, $2)",
                data.id,
                player.id,
            )
            .execute(&mut *conn)
            .await?;
        }

        Ok(Self {
            id: data.id,
            scenario,
            state,
            players,
        })
    }

    pub async fn load(
        id: Uuid,
        conn: impl Executor<'_, Database = Postgres>,
    ) -> anyhow::Result<Self> {
        sqlx::query_as!(
            RawGame,
            r#"
            SELECT g.*, array_agg(p.name) as players
                FROM games g
                LEFT OUTER JOIN game_players gp ON gp.game_id = g.id
                LEFT OUTER JOIN players p ON gp.player_id = p.id
                WHERE g.id = $1
                GROUP BY g.id
            "#,
            id
        )
        .fetch_one(conn)
        .await?
        .try_into()
    }

    pub async fn load_many(
        id: Option<Uuid>,
        limit: i64,
        asc: bool,
        conn: impl Executor<'_, Database = Postgres>,
    ) -> anyhow::Result<Vec<Self>> {
        let data = if let Some(id) = id {
            if asc {
                sqlx::query_as!(
                    RawGame,
                    r#"
                    SELECT g.*, array_agg(p.name) as players
                        FROM games g
                        LEFT OUTER JOIN game_players gp ON gp.game_id = g.id
                        LEFT OUTER JOIN players p ON gp.player_id = p.id
                        WHERE g.id > $1
                        GROUP BY g.id
                        ORDER BY g.id ASC
                        LIMIT $2
                    "#,
                    id,
                    limit,
                )
                .fetch_all(conn)
                .await?
            } else {
                sqlx::query_as!(
                    RawGame,
                    r#"
                    SELECT g.*, array_agg(p.name) as players
                        FROM games g
                        LEFT OUTER JOIN game_players gp ON gp.game_id = g.id
                        LEFT OUTER JOIN players p ON gp.player_id = p.id
                        WHERE g.id > $1
                        GROUP BY g.id
                        ORDER BY g.id DESC
                        LIMIT $2
                    "#,
                    id,
                    limit,
                )
                .fetch_all(conn)
                .await?
            }
        } else if asc {
            sqlx::query_as!(
                RawGame,
                r#"
                    SELECT g.*, array_agg(p.name) as players
                        FROM games g
                        LEFT OUTER JOIN game_players gp ON gp.game_id = g.id
                        LEFT OUTER JOIN players p ON gp.player_id = p.id
                        GROUP BY g.id
                        ORDER BY g.id ASC
                        LIMIT $1
                    "#,
                limit,
            )
            .fetch_all(conn)
            .await?
        } else {
            sqlx::query_as!(
                RawGame,
                r#"
                    SELECT g.*, array_agg(p.name) as players
                        FROM games g
                        LEFT OUTER JOIN game_players gp ON gp.game_id = g.id
                        LEFT OUTER JOIN players p ON gp.player_id = p.id
                        GROUP BY g.id
                        ORDER BY g.id DESC
                        LIMIT $1
                    "#,
                limit,
            )
            .fetch_all(conn)
            .await?
        };
        data.into_iter()
            .map(Game::try_from)
            .collect::<Result<Vec<_>, _>>()
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
            players: raw.players.unwrap_or_default(),
        })
    }
}
