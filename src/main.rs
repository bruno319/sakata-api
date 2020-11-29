#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use base_card::handlers::*;

use crate::dbconfig::connect;

#[macro_use]
mod macros;
mod model;
mod schema;
mod dbconfig;
mod dto;
mod base_card;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(connect())
            .service(get_cards)
            .service(create_base_card)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}