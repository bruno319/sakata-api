use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};

use crate::dbconfig::MysqlPool;
use crate::dto::{AnimeIdsDto, BaseCardDto};
use crate::utils::{extract_path_param, http_res, mysql_pool_handler};

use super::{BaseCard, calc_overall_power};
use super::dao;

#[get("/basecards")]
pub async fn get_cards(pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    match dao::list(&mysql_pool) {
        Ok(bc) => Ok(http_res::ok(bc)),
        Err(e) => Err(http_res::internal_server_error(&e.to_string())),
    }
}

#[get("/basecards/{id}")]
pub async fn get_card_by_id(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_card_id: i32 = extract_path_param("id", &req)?;

    match dao::find_by_id(&mysql_pool, base_card_id) {
        Ok(bc) => Ok(http_res::ok(bc)),
        Err(e) => Err(http_res::internal_server_error(&e.to_string())),
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
        .map_err(|e| http_res::internal_server_error(&e.to_string()))?;

    match dao::save(&mysql_pool, &base_card) {
        Ok(bc) => Ok(http_res::ok(bc)),
        Err(e) => Err(http_res::internal_server_error(&e.to_string())),
    }
}

#[post("/basecards/overall-power/{mal_id}")]
pub async fn generate_overall_power(
    req: HttpRequest,
    animes: web::Json<AnimeIdsDto>,
) -> Result<HttpResponse, HttpResponse> {
    let mal_id: u32 = extract_path_param("mal_id", &req)?;

    let overall_power = calc_overall_power(mal_id, animes.0.anime_mal_ids)
        .await
        .map_err(|e| http_res::internal_server_error(&e.to_string()))?;

    Ok(http_res::ok(serde_json::json!({"overallPower": overall_power})))
}