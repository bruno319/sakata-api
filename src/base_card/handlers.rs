use actix_multipart::Multipart;
use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};
use serde_json::json;

use crate::dbconfig::MysqlPool;
use crate::s3::AwsS3Client;
use crate::types::json_req::{AnimesJson, BaseCardJson};
use crate::utils::{extract_path_param, http_res, image_bytes, mysql_pool_handler};

use super::{BaseCard, calc_overall_power, dao};

#[get("/basecards")]
pub async fn get_cards(pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_cards = dao::list(&mysql_pool)?;
    Ok(http_res::ok(base_cards))
}

#[get("/basecards/{id}")]
pub async fn get_card_by_id(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_card_id = extract_path_param("id", &req)?;
    let base_card = dao::find_by_id(&mysql_pool, base_card_id)?;
    Ok(http_res::ok(base_card))
}

#[post("/basecards")]
pub async fn create_base_card(
    base_card_dto: web::Json<BaseCardJson>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_card = BaseCard::new(base_card_dto.0);
    let base_card = dao::save(&mysql_pool, &base_card)?;
    Ok(http_res::created(base_card))
}

#[post("/basecards/overall-power/{mal_id}")]
pub async fn generate_overall_power(
    req: HttpRequest,
    animes: web::Json<AnimesJson>,
) -> Result<HttpResponse, HttpResponse> {
    let mal_id = extract_path_param("mal_id", &req)?;
    let overall_power = calc_overall_power(mal_id, animes.0.animes).await?;
    Ok(http_res::ok(json!({"overall_power": overall_power})))
}

#[post("/basecards/image")]
pub async fn save_image_card(payload: Multipart) -> Result<HttpResponse, HttpResponse> {
    let bytes = image_bytes(payload).await?;
    let url = AwsS3Client::new().put_object(bytes.0, bytes.1).await?;
    Ok(http_res::created(json!({"url": url})))
}
