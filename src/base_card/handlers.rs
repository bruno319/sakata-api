use std::borrow::BorrowMut;

use actix_multipart::{Field, Multipart};
use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};
use futures::StreamExt;
use log::*;

use crate::dbconfig::MysqlPool;
use crate::dto::{AnimeIdsDto, BaseCardDto};
use crate::error::SakataError;
use crate::s3::AwsS3Client;
use crate::SakataResult;
use crate::utils::{extract_path_param, http_res, mysql_pool_handler};
use crate::utils::http_res::not_found;

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
    base_card_dto: web::Json<BaseCardDto>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let base_card = BaseCard::new(base_card_dto.0);
    let base_card = dao::save(&mysql_pool, &base_card)?;
    Ok(http_res::ok(base_card))
}

#[post("/basecards/overall-power/{mal_id}")]
pub async fn generate_overall_power(
    req: HttpRequest,
    animes: web::Json<AnimeIdsDto>,
) -> Result<HttpResponse, HttpResponse> {
    let mal_id = extract_path_param("mal_id", &req)?;

    let overall_power = calc_overall_power(mal_id, animes.0.anime_mal_ids).await?;
    Ok(http_res::ok(serde_json::json!({"overall_power": overall_power})))
}

#[post("/basecards/image")]
pub async fn save_image_card(mut payload: Multipart) -> Result<HttpResponse, HttpResponse> {
    let bytes = image_bytes(payload.borrow_mut()).await?;
    let url = AwsS3Client::new().put_object(bytes.0, bytes.1).await;
    info!("{} uploaded to S3", url);
    Ok(HttpResponse::Ok().into())
}

pub async fn image_bytes(payload: &mut Multipart) -> SakataResult<(Vec<u8>, String)> {
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect(" split_payload err");
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap_or_default();
        if name == "basecard" {
            if let Some(filename) = content_type.get_filename() {
                let mut output = Vec::new();

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap().to_vec();
                    output.extend(data);
                }
                return Ok((output, filename.to_string()));
            }
        }
    }
    Err(SakataError::ResourceNotFound(not_found("file not found")))
}
