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
    let player_id = extract_path_param("id", &req)?;
    let player = dao::find_by_id(&mysql_pool, player_id)?;
    Ok(http_res::ok(player))
}

#[post("/players")]
pub async fn create_player(
    player_dto: web::Json<PlayerDto>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player = Player::new(player_dto.0);
    let player = dao::save(&mysql_pool, &player)?;
    Ok(http_res::created(player))
}

#[get("/players/{discord_id}/common-card")]
pub async fn buy_common_card(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let mut player = dao::find_by_discord_id(&mysql_pool, player_id)?;

    let base_card = player.buy_common_card(&mysql_pool)?;
    let player_card = player_card::add_to_collection(&player, &base_card, &mysql_pool)?;
    Ok(http_res::ok(PlayerCardResponse::new(&player_card, &base_card)))
}

#[get("/players/{discord_id}/star-card")]
pub async fn buy_star_card(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let mut player = dao::find_by_discord_id(&mysql_pool, player_id)?;

    let base_card = player.buy_star_card(&mysql_pool)?;
    let player_card = player_card::add_to_collection(&player, &base_card, &mysql_pool)?;
    Ok(http_res::ok(PlayerCardResponse::new(&player_card, &base_card)))
}