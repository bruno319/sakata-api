use actix_web::{get, HttpRequest, HttpResponse, post, web};

use crate::{base_card, party, player, player_card};
use crate::dbconfig::MysqlPool;
use crate::party::Party;
use crate::player::Player;
use crate::types::json_req::{PlayerJson, SwapPartyCardsJson};
use crate::types::json_res::{PlayerJoinedResponse, PartyResponse, PlayerCardResponse};
use crate::utils::{extract_path_param, http_res, mysql_pool_handler};

#[get("/players/{id}")]
pub async fn get_player_by_id(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("id", &req)?;
    let player = player::dao::find_by_id(&mysql_pool, player_id)?;
    Ok(http_res::ok(player))
}

#[post("/players")]
pub async fn create_player(
    player_json: web::Json<PlayerJson>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let mut player = player::dao::save(&mysql_pool, &Player::new(player_json.0))?;

    let mut initial_cards = Vec::with_capacity(5);
    let mut base_card_ids = Vec::with_capacity(5);
    while initial_cards.len() < 5 {
        let base_card = player.buy_common_card(&mysql_pool)?;
        if base_card_ids.contains(&base_card.id.unwrap()) {
            player.coins += 50;
            player::dao::update_coins(&player, &mysql_pool)?;
            continue;
        };
        base_card_ids.push(base_card.id.unwrap());
        let player_card = player_card::add_to_collection(&player, &base_card, &mysql_pool)?;
        initial_cards.push((player_card, base_card));
    }

    let party = Party::new(player.discord_id, initial_cards);
    party::dao::save(&mysql_pool, &party)?;

    let player_res = PlayerJoinedResponse::new(player, party);
    Ok(http_res::created(player_res))
}

#[get("/players/{discord_id}/common-card")]
pub async fn buy_common_card(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let mut player = player::dao::find_by_discord_id(&mysql_pool, player_id)?;

    let base_card = player.buy_common_card(&mysql_pool)?;
    let player_card = player_card::add_to_collection(&player, &base_card, &mysql_pool)?;
    Ok(http_res::ok(PlayerCardResponse::new(player_card, base_card)))
}

#[get("/players/{discord_id}/star-card")]
pub async fn buy_star_card(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let mut player = player::dao::find_by_discord_id(&mysql_pool, player_id)?;

    let base_card = player.buy_star_card(&mysql_pool)?;
    let player_card = player_card::add_to_collection(&player, &base_card, &mysql_pool)?;
    Ok(http_res::ok(PlayerCardResponse::new(player_card, base_card)))
}

#[get("/players/{discord_id}/party")]
pub async fn get_party(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let party = party::dao::find_by_discord_id(&mysql_pool, player_id)?;
    Ok(HttpResponse::Ok().json(PartyResponse::new(party)))
}

#[post("/players/{discord_id}/party/swap")]
pub async fn swap_party_cards(
    req: HttpRequest,
    swap: web::Json<SwapPartyCardsJson>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let player = player::dao::find_by_discord_id(&mysql_pool, player_id)?;
    let card_in = base_card::dao::find_by_mal_id(&mysql_pool, swap.card_in)?;
    let card_out = base_card::dao::find_by_mal_id(&mysql_pool, swap.card_out)?;
    let mut party = party::dao::find_by_discord_id(&mysql_pool, player_id)?;

    party.swap(card_in, card_out, player, &mysql_pool)?;
    party::dao::update(&mysql_pool, &party)?;

    Ok(HttpResponse::Ok().json(PartyResponse::new(party)))
}