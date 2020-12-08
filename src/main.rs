#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;

use crate::dbconfig::connect;

#[macro_use]
mod macros;
mod model;
mod schema;
mod dbconfig;
mod dto;
mod base_card;
mod player;
mod player_card;
mod utils;
mod error;
mod s3;

type SakataResult<T> = Result<T, error::SakataError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .data(connect())
            .service(base_card::handlers::save_image_card)
            .service(base_card::handlers::get_cards)
            .service(base_card::handlers::create_base_card)
            .service(base_card::handlers::generate_overall_power)
            .service(base_card::handlers::get_card_by_id)
            .service(player::handlers::create_player)
            .service(player::handlers::get_player_by_id)
            .service(player::handlers::buy_common_card)
            .service(player::handlers::buy_star_card)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}