use lazy_static::lazy_static;
use std::net::IpAddr;

lazy_static! {
    pub static ref HOST: IpAddr = std::env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_owned())
        .parse()
        .unwrap();
    pub static ref PORT: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_owned())
        .parse()
        .unwrap();
}
