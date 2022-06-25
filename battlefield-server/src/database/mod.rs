pub use sqlx::postgres::PgPool;

mod game;

pub use game::{Game, RawGame};

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    log::debug!("Connecting to database at {url}");
    Ok(PgPool::connect(url).await?)
}
