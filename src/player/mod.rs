use serde::{Deserialize, Serialize};

use crate::{base_card, SakataResult};
use crate::base_card::BaseCard;
use crate::dbconfig::MySqlPooledConnection;
use crate::dto::PlayerDto;
use crate::schema::players;
use crate::error::SakataError;
use crate::utils::http_res::forbidden;

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
            stardust: 50,
        }
    }

    pub fn buy_common_card(&mut self, conn: &MySqlPooledConnection) -> SakataResult<BaseCard> {
        if self.coins < 50 {
            return Err(SakataError::NotEnoughResource(forbidden("Insufficient Coins")));
        }

        self.coins -= 50;
        dao::update_coins(&self, conn)?;

        base_card::common_card(&conn)
    }

    pub fn buy_star_card(&mut self, conn: &MySqlPooledConnection) -> SakataResult<BaseCard> {
        if self.stardust < 50 {
            return Err(SakataError::NotEnoughResource(forbidden("Insufficient Stardust")));
        }

        self.stardust -= 50;
        dao::update_stardust(&self, conn)?;

        base_card::star_card(&conn)
    }
}