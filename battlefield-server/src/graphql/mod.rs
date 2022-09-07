use actix_web::{web, Scope};
use actix_web::{HttpRequest, HttpResponse};

mod context;
mod scalars;
mod schema;

use context::Context;
use juniper_actix::{graphql_handler, playground_handler};
use scalars::Json;
pub use schema::{schema, Schema};

#[actix_web::get("playground")]
async fn playground() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    context: Context,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    graphql_handler(&schema, &context, req, payload).await
}

pub fn service() -> Scope {
    web::scope("graphql")
        .service(
            web::resource("")
                .app_data(web::Data::new(schema()))
                .route(web::get().to(graphql))
                .route(web::post().to(graphql)),
        )
        .service(playground)
}
