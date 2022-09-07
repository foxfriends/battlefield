use actix_web::web::{self, ServiceConfig};

mod connect;
mod handler;

use connect::connect;
pub(crate) use handler::{Notification, SocketHandler};

pub fn configure(config: &mut ServiceConfig) {
    config.route("/ws/{game_id}", web::get().to(connect));
}
