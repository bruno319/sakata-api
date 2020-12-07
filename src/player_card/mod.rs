use std::env;

use diesel::MysqlConnection;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

use crate::base_card::BaseCard;
use crate::model::{Rarity, Class, Genre};
use crate::player::Player;
use crate::schema::player_cards;
use crate::SakataResult;

mod dao;

#[derive(Queryable, Identifiable, Insertable, Associations, Serialize, Deserialize, Clone, Copy, Debug)]
#[belongs_to(Player, foreign_key = "id")]
#[belongs_to(BaseCard, foreign_key = "id")]
pub struct PlayerCard {
    id: Option<i32>,
    base_card_id: i32,
    player_id: i32,
    rarity: Rarity,
    quantity: i8,
}

impl PlayerCard {
    pub fn new(player: &Player, base_card: &BaseCard, rarity: Rarity) -> PlayerCard {
        PlayerCard {
            id: None,
            base_card_id: base_card.id.unwrap(),
            player_id: player.id.unwrap(),
            rarity,
            quantity: 1,
        }
    }
}

fn generate_rarity() -> Rarity {
    let rand = thread_rng().gen_range(0, 101);

    if rand < 4 {
        Rarity::Legend
    } else if rand < 12 {
        Rarity::Epic
    } else if rand < 40 {
        Rarity::Gold
    } else {
        Rarity::Silver
    }
}

pub fn add_to_collection(player: &Player, base_card: &BaseCard, conn: &MysqlConnection) -> SakataResult<PlayerCard> {
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

    dao::save(&PlayerCard::new(player, base_card, rarity), conn)
        .map(|pc| *pc)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerCardResponse {
    pub card_id: i32,
    pub name: String,
    pub rarity: Rarity,
    pub class: Class,
    pub genre: Genre,
    pub image_url: String,
    pub quantity: i8,
}

impl PlayerCardResponse {
    pub fn new(player_card: &PlayerCard, base_card: &BaseCard) -> PlayerCardResponse {
        let image_name = format!("sakata_{}[{}].jpeg", base_card.mal_id, player_card.rarity as i8);
        let image_url = format!("{}/{}", env::var("IMAGE_BASEURL").unwrap_or_default(), image_name);

        PlayerCardResponse {
            card_id: base_card.id.unwrap_or_default(),
            name: base_card.name.clone(),
            rarity: player_card.rarity,
            class: base_card.class,
            genre: base_card.genre,
            image_url,
            quantity: player_card.quantity,
        }
    }
}