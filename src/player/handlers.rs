use actix_web::{get, HttpRequest, HttpResponse, post, web};

use crate::{base_card, party, player, player_card};
use crate::base_card::drawer::BaseCardDrawer;
use crate::dbconfig::MysqlPool;
use crate::party::Party;
use crate::player::Player;
use crate::types::json_req::{PlayerCardQuery, PlayerJson, SwapPartyCardsJson};
use crate::types::json_res::{PartyResponse, PlayerCardResponse, PlayerJoinedResponse};
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
    drawer: web::Data<BaseCardDrawer>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let mut player = player::dao::save(&mysql_pool, &Player::new(player_json.0))?;

    let mut initial_cards = Vec::with_capacity(5);
    let mut bc_id_list = Vec::with_capacity(5);
    while initial_cards.len() < 5 {
        let (pc, bc) = player.buy_common_card(&drawer, &mysql_pool)?;
        if bc_id_list.contains(&bc.id.unwrap()) {
            player.coins += 50;
            player::dao::update_coins(&player, &mysql_pool)?;
            player_card::dao::remove_by_id(pc.id, &mysql_pool)?;
            continue;
        };
        bc_id_list.push(bc.id.unwrap());
        initial_cards.push((pc, bc));
    }

    let party = Party::new(player.discord_id, initial_cards);
    party::dao::save(&mysql_pool, &party)?;

    let player_res = PlayerJoinedResponse::new(player, party);
    Ok(http_res::created(player_res))
}

#[get("/players/{discord_id}/common-card")]
pub async fn buy_common_card(
    req: HttpRequest,
    drawer: web::Data<BaseCardDrawer>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let mut player = player::dao::find_by_discord_id(&mysql_pool, player_id)?;

    let (player_card, base_card) = player.buy_common_card(&drawer, &mysql_pool)?;
    Ok(http_res::ok(PlayerCardResponse::new(player_card, base_card)))
}

#[get("/players/{discord_id}/star-card")]
pub async fn buy_star_card(
    req: HttpRequest,
    drawer: web::Data<BaseCardDrawer>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let mut player = player::dao::find_by_discord_id(&mysql_pool, player_id)?;

    let (player_card, base_card) = player.buy_star_card(&drawer, &mysql_pool)?;
    Ok(http_res::ok(PlayerCardResponse::new(player_card, base_card)))
}

#[get("/players/{discord_id}/cards")]
pub async fn query_player_cards(
    req: HttpRequest,
    query: web::Query<PlayerCardQuery>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let player = player::dao::find_by_discord_id(&mysql_pool, player_id)?;
    let cards = player_card::query(player, query.0, &mysql_pool)?;

    Ok(http_res::ok(cards))
}

#[get("/players/{discord_id}/party")]
pub async fn get_party(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id = extract_path_param("discord_id", &req)?;
    let party = party::dao::find_by_discord_id(&mysql_pool, player_id)?;
    Ok(http_res::ok(PartyResponse::new(party)))
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

    Ok(http_res::ok(PartyResponse::new(party)))
}