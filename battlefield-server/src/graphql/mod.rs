use actix_web::web::{self, Payload, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};

mod context;
mod schema;

use context::Context;
use juniper_actix::{graphql_handler, playground_handler};
use schema::{schema, Schema};

async fn playground() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

<<<<<<< Updated upstream
async fn graphql(req: HttpRequest, payload: Payload, context: Context, schema: web::Data<Schema>) -> actix_web::Result<HttpResponse> {
=======
async fn graphql(
    req: HttpRequest,
    payload: Payload,
    context: Context,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
>>>>>>> Stashed changes
    graphql_handler(&schema, &context, req, payload).await
}

pub fn configure(config: &mut ServiceConfig) {
<<<<<<< Updated upstream
    config.service(
        web::resource("/graphql")
            .app_data(web::Data::new(schema()))
            .route(web::get().to(graphql))
            .route(web::post().to(graphql)),
    )
    .route("/graphql/playground", web::get().to(playground));
=======
    config
        .service(
            web::resource("/graphql")
                .app_data(web::Data::new(schema()))
                .route(web::get().to(graphql))
                .route(web::post().to(graphql)),
        )
        .route("/graphql/playground", web::get().to(playground));
>>>>>>> Stashed changes
}
