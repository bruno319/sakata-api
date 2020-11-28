#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;

use crate::dbconfig::connect;

#[macro_use]
mod macros;
mod model;
mod schema;
mod dbconfig;
mod handlers;
mod dao;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(connect())
            .service(web::resource("/").route(web::get().to(handlers::get)))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .await
}