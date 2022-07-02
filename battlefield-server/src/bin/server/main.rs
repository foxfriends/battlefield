use actix_web::{middleware, App, HttpServer};
use battlefield_core::EngineBuilder;
use battlefield_server::BattlefieldServer;

mod env;
mod filter_middleware;

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
        .add_maps(&*env::MAPS_DIR)
        .build();
    let battlefield = BattlefieldServer::new(&env::DATABASE_URL, engine).await?;

    HttpServer::new(move || {
        let logger = middleware::Logger::default().log_target("[http request]");
        let logger = filter_middleware::Filter::new(middleware::Compat::new(logger), |req| {
            req.headers()
                .get("Referer")
                .and_then(|s| s.to_str().ok())
                .map(|s| !s.contains("/graphql/playground"))
                .unwrap_or(true)
        });
        App::new()
            .wrap(logger)
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(|config| battlefield.configure(config))
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
