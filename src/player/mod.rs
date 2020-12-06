use serde::{Deserialize, Serialize};

use crate::base_card;
use crate::base_card::BaseCard;
use crate::dbconfig::MySqlPooledConnection;
use crate::dto::PlayerDto;
use crate::schema::players;

mod dao;
pub mod handlers;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "players"]
pub struct Player {
    pub id: Option<i32>,
    pub nickname: String,
    pub coins: i16,
    pub stardust: i16,
    pub discord_id: i64,
}

impl Player {
    pub fn new(dto: PlayerDto) -> Player {
        Player {
            id: None,
            discord_id: dto.discord_id,
            nickname: dto.nickname,
            coins: 300,
            stardust: 0,
        }
    }

    pub fn buy_common_card(&mut self, conn: &MySqlPooledConnection) -> Result<BaseCard, String> {
        if self.coins < 50 {
            return Err("Insufficient Coins".to_string());
        }

        self.coins -= 50;
        dao::update_coins(&self, conn)
            .map_err(|e| e.to_string())?;

        base_card::common_card(&conn)
    }
}