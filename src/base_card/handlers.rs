use actix_web::{get, HttpResponse, post, Result, web};

use crate::base_card::BaseCard;
use crate::base_card::dao::BaseCardDao;
use crate::dbconfig::{MysqlPool, MySqlPooledConnection};
use crate::dto::BaseCardDto;

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[get("/basecards")]
pub async fn get_cards(pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    match BaseCardDao::list(&mysql_pool) {
        Ok(c) => Ok(HttpResponse::Ok().json(c)),
        Err(e) => Err(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[post("/basecards")]
pub async fn create_base_card(
    base_card_dto: web::Json<BaseCardDto>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_card = BaseCard::new(base_card_dto.0)
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))?;

    match BaseCardDao::insert(&mysql_pool, &base_card) {
        Ok(bc) => Ok(HttpResponse::Ok().json(bc)),
        Err(e) => Err(HttpResponse::InternalServerError().json(e.to_string())),
    }
}