use actix_web::{middleware, App, HttpServer};
use battlefield_core::EngineBuilder;
use battlefield_server::BattlefieldServer;

mod env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let host = *env::HOST;
    let port = *env::PORT;

    log::info!("Battlefield served on {}:{}", host, port);

    let engine = EngineBuilder::new()
        .add_scenarios(&*env::SCENARIOS_DIR)
        .add_modules(&*env::MODULES_DIR)
        .build();
    let battlefield = BattlefieldServer::new(&env::DATABASE_URL, engine).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(|config| battlefield.configure(config))
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
