use actix_web::{middleware, App, HttpServer};
use battlefield_server::configure;

mod env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let host = *env::HOST;
    let port = *env::PORT;

    log::info!("Battlefield served on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(configure)
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
