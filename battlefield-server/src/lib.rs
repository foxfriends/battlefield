use actix_web::web::{self, ServiceConfig};

mod socket;

pub fn configure(config: &mut ServiceConfig) {
    config.route("/ws", web::get().to(socket::connect));
}
