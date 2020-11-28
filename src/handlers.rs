use actix_web::{HttpRequest, HttpResponse};
use actix_web::web;

use crate::dao::Cards;
use crate::dbconfig::{MysqlPool, MySqlPooledConnection};

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub async fn get(
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    match Cards::list(&mysql_pool) {
        Ok(c) => Ok(HttpResponse::Ok().json(c)),
        Err(e) => Err(HttpResponse::InternalServerError().json(e.to_string())),
    }
}