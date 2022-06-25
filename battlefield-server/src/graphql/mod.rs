use actix_web::web::{self, Payload, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};

mod context;
mod scalars;
mod schema;

use context::Context;
use juniper_actix::{graphql_handler, playground_handler};
use scalars::Json;
use schema::{schema, Schema};

async fn playground() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

async fn graphql(
    req: HttpRequest,
    payload: Payload,
    context: Context,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    graphql_handler(&schema, &context, req, payload).await
}

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(
            web::resource("/graphql")
                .app_data(web::Data::new(schema()))
                .route(web::get().to(graphql))
                .route(web::post().to(graphql)),
        )
        .route("/graphql/playground", web::get().to(playground));
}
