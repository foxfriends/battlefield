pub use sqlx::postgres::PgPool;

mod game;
mod player;

pub use game::Game;
pub use player::Player;

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    log::debug!("Connecting to database at {url}");
    Ok(PgPool::connect(url).await?)
}
