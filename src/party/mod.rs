use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};
use crate::player::Player;

use crate::schema::party;

#[derive(Queryable, Identifiable, Insertable, Associations, Serialize, Deserialize, Clone, Copy, Debug)]
#[belongs_to(Player, foreign_key = "id")]
#[table_name = "party"]
pub struct Party {
    pub id: u32,
    pub power: u16,
    pub card_1: u32,
    pub card_2: u32,
    pub card_3: u32,
    pub card_4: u32,
    pub card_5: u32,
}