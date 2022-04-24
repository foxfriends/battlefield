pub use sqlx::postgres::PgPool;

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    log::debug!("Connecting to database at {url}");
    Ok(PgPool::connect(url).await?)
}
