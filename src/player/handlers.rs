use actix_web::{get, HttpRequest, HttpResponse, post, web};

use crate::dbconfig::MysqlPool;
use crate::dto::PlayerDto;
use crate::player::Player;
use crate::player_card;
use crate::player_card::PlayerCardResponse;
use crate::utils::{extract_path_param, http_res, mysql_pool_handler};

use super::dao;

#[get("/players/{id}")]
pub async fn get_player_by_id(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id: i32 = extract_path_param("id", &req)?;

    match dao::find_by_id(&mysql_pool, player_id) {
        Ok(p) => Ok(http_res::ok(p)),
        Err(e) => Err(http_res::internal_server_error(&e.to_string())),
    }
}

#[post("/players")]
pub async fn create_player(
    player_dto: web::Json<PlayerDto>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player = Player::new(player_dto.0);

    match dao::save(&mysql_pool, &player) {
        Ok(p) => Ok(http_res::ok(p)),
        Err(e) => Err(http_res::internal_server_error(&e.to_string())),
    }
}

#[get("/players/{discord_id}/common-card")]
pub async fn buy_common_card(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id: i64 = extract_path_param("discord_id", &req)?;

    let mut player = dao::find_by_discord_id(&mysql_pool, player_id)
        .map_err(|e| http_res::internal_server_error(&e.to_string()))?;

    let base_card = player.buy_common_card(&mysql_pool)
        .map_err(|e| http_res::internal_server_error(&e))?;

    let player_card = player_card::add_to_collection(&player, &base_card, &mysql_pool)
        .map_err(|e| http_res::internal_server_error(&e.to_string()))?;

    Ok(HttpResponse::Ok().json(PlayerCardResponse::new(&player_card, &base_card)))
}