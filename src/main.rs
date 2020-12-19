#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use crate::base_card::drawer::BaseCardDrawer;

#[macro_use]
mod macros;
mod types;
mod schema;
mod dbconfig;
mod base_card;
mod player;
mod player_card;
mod utils;
mod error;
mod s3;
mod party;

type SakataResult<T> = Result<T, error::SakataError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let conn = dbconfig::connect();
        let drawer = BaseCardDrawer::new(&conn.get().unwrap())
            .expect("Cannot fetch card data in mysql");

        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .data(conn)
            .data(drawer)
            .service(base_card::handlers::save_image_card)
            .service(base_card::handlers::get_cards)
            .service(base_card::handlers::verify_inserted)
            .service(base_card::handlers::create_base_card)
            .service(base_card::handlers::generate_overall_power)
            .service(base_card::handlers::get_card_by_id)
            .service(player::handlers::create_player)
            .service(player::handlers::get_player_by_id)
            .service(player::handlers::buy_common_card)
            .service(player::handlers::buy_star_card)
            .service(player::handlers::query_player_cards)
            .service(player::handlers::get_party)
            .service(player::handlers::swap_party_cards)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}