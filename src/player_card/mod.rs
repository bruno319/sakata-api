use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::base_card::BaseCard;
use crate::dbconfig::MySqlPooledConnection;
use crate::player::Player;
use crate::SakataResult;
use crate::schema::player_cards;
use crate::types::model::Rarity;

pub mod dao;

#[derive(Queryable, QueryableByName, Identifiable, Insertable, Associations, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[table_name = "player_cards"]
#[belongs_to(Player, foreign_key = "id")]
#[belongs_to(BaseCard, foreign_key = "id")]
pub struct PlayerCard {
    pub id: String,
    pub base_card_id: u32,
    pub player_id: u32,
    pub rarity: Rarity,
    pub quantity: u8,
}

impl PlayerCard {
    pub fn new(player: &Player, base_card: &BaseCard, rarity: Rarity) -> PlayerCard {
        PlayerCard {
            id: Uuid::new_v4().to_string(),
            base_card_id: base_card.id.unwrap(),
            player_id: player.id.unwrap(),
            rarity,
            quantity: 1,
        }
    }
}

fn generate_rarity() -> Rarity {
    let rand = thread_rng().gen_range(0, 101);

    if rand < 5 {
        Rarity::Legend
    } else if rand < 15 {
        Rarity::Epic
    } else if rand < 50 {
        Rarity::Gold
    } else {
        Rarity::Silver
    }
}

pub fn add_to_collection(player: &Player, base_card: &BaseCard, conn: &MySqlPooledConnection) -> SakataResult<PlayerCard> {
    let player_cards = dao::find_by(player, base_card, conn);
    let rarity = generate_rarity();

    if let Ok(p_cards) = player_cards {
        for mut pc in p_cards {
            if pc.rarity == rarity {
                pc.quantity += 1;
                dao::update_quantity(&pc, conn)?;
                return Ok(pc);
            }
        }
    }

    let player = PlayerCard::new(player, base_card, rarity);
    dao::save(&player, conn)?;

    Ok(player)
}