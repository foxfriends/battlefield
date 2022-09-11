use actix_web::{web, Scope};

mod connect;
mod handler;

pub(crate) use handler::{SocketHandler, Update};

pub fn service() -> Scope {
    web::scope("ws").service(connect::handler)
}
