use actix_web::{get, HttpResponse, post, Result, web, HttpRequest};

use crate::base_card::{BaseCard, calc_overall_power};
use crate::base_card::dao::BaseCardDao;
use crate::dbconfig::{MysqlPool, MySqlPooledConnection};
use crate::dto::{BaseCardDto, AnimeIdsDto};

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

#[get("/basecards/{id}")]
pub async fn get_card_by_id(
    req: HttpRequest,
    pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse>{
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_card_id: String = req.match_info().get("id")
        .ok_or(HttpResponse::BadRequest().json("ID not provided"))?
        .parse()
        .map_err(|_| HttpResponse::BadRequest().json("ID must be a String"))?;

    match BaseCardDao::find_by_id(&mysql_pool, base_card_id){
        Ok(bc) => Ok(HttpResponse::Ok().json(bc)),
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

#[post("/basecards/overall-power/{mal_id}")]
pub async fn generate_overall_power(
    req: HttpRequest,
    animes: web::Json<AnimeIdsDto>,
    _pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mal_id: u32 = req.match_info().get("mal_id")
        .ok_or(HttpResponse::BadRequest().json("MAL ID not provided"))?
        .parse()
        .map_err(|_| HttpResponse::BadRequest().json("MAL ID must be a number"))?;

    let overall_power = calc_overall_power(mal_id, animes.0.anime_mal_ids)
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"overallPower": overall_power})))
}