use serde::{Deserialize, Serialize};

use crate::base_card::BaseCard;
use crate::model::Rarity;
use crate::player::Player;
use crate::schema::player_cards;

#[derive(Queryable, Identifiable, Insertable, Associations, Serialize, Deserialize)]
#[belongs_to(Player)]
#[belongs_to(BaseCard)]
pub struct PlayerCard {
    id: i32,
    base_card_id: i32,
    player_id: i32,
    rarity: Rarity,
    quantity: i8,
}