use sqlx::{Executor, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

impl Player {
    pub async fn find_or_create<C>(name: &str, conn: &mut C) -> anyhow::Result<Self>
    where
        for<'a> &'a mut C: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query_as!(
            Self,
            "INSERT INTO players (name) values ($1) ON CONFLICT DO NOTHING RETURNING *",
            name,
        )
        .fetch_one(conn)
        .await?)
    }
}
