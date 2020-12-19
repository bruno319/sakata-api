use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::{base_card, player_card, SakataResult};
use crate::base_card::{BaseCard, rawer::BaseCardDrawer};
use crate::dbconfig::MySqlPooledConnection;
use crate::error::SakataError;
use crate::player_card::PlayerCard;
use crate::schema::players;
use crate::types::json_req::PlayerJson;

mod dao;
pub mod handlers;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "players"]
pub struct Player {
    pub id: Option<u32>,
    pub discord_id: i64,
    pub channel_id: i64,
    pub nickname: String,
    pub discriminator: u16,
    pub coins: i16,
    pub stardust: i16,
}

impl Player {
    pub fn new(json: PlayerJson) -> Player {
        Player {
            id: None,
            discord_id: json.discord_id,
            channel_id: json.channel_id,
            nickname: json.nickname,
            discriminator: json.discriminator,
            coins: 350,
            stardust: 50,
        }
    }

    pub fn buy_common_card(&mut self, drawer: &web::Data<BaseCardDrawer>, conn: &MySqlPooledConnection) -> SakataResult<(PlayerCard, BaseCard)> {
        if self.coins < 50 {
            return Err(SakataError::NotEnoughResource("Insufficient Coins".to_string()));
        }
        self.coins -= 50;
        dao::update_coins(&self, conn)?;

        let base_card = base_card::dao::find_by_id(conn, drawer.common_card())?;
        let player_card = player_card::add_to_collection(self, &base_card, conn)?;
        Ok((player_card, base_card))
    }

    pub fn buy_star_card(&mut self, drawer: &web::Data<BaseCardDrawer>, conn: &MySqlPooledConnection) -> SakataResult<(PlayerCard, BaseCard)> {
        if self.stardust < 50 {
            return Err(SakataError::NotEnoughResource("Insufficient Stardust".to_string()));
        }
        self.stardust -= 50;
        dao::update_stardust(&self, conn)?;

        let base_card = base_card::dao::find_by_id(conn, drawer.star_card())?;
        let player_card = player_card::add_to_collection(self, &base_card, conn)?;
        Ok((player_card, base_card))
    }
}