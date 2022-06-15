use actix_web::web::{self, ServiceConfig};

mod connect;
mod create;
mod handler;

use connect::connect;
use create::create;
pub use handler::{Notification, SocketHandler};

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/ws/new/{scenario}", web::get().to(create))
        .route("/ws/{game_id}", web::get().to(connect));
}
