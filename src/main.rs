#[macro_use]
extern crate diesel;

use actix_cors::Cors;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .data(connect())
            .service(get_cards)
            .service(create_base_card)
            .service(generate_overall_power)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}