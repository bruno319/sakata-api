use serde::{Deserialize, Serialize};

use crate::{player_card, SakataResult};
use crate::base_card::BaseCard;
use crate::dbconfig::MySqlPooledConnection;
use crate::error::SakataError;
use crate::player::Player;
use crate::player_card::PlayerCard;
use crate::schema::party;

pub mod dao;

#[derive(Queryable, Identifiable, Insertable, Associations, Serialize, Deserialize, Clone, Debug, Default)]
#[belongs_to(Player, foreign_key = "id")]
#[table_name = "party"]
pub struct PartyEntity {
    pub id: i64,
    pub power: u16,
    pub card_1: String,
    pub card_2: String,
    pub card_3: String,
    pub card_4: String,
    pub card_5: String,
}

impl From<&Party> for PartyEntity {
    fn from(party: &Party) -> PartyEntity {
        PartyEntity {
            id: party.id,
            power: party.power,
            card_1: party.cards[0].0.id.clone(),
            card_2: party.cards[1].0.id.clone(),
            card_3: party.cards[2].0.id.clone(),
            card_4: party.cards[3].0.id.clone(),
            card_5: party.cards[4].0.id.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Party {
    pub id: i64,
    pub power: u16,
    pub cards: Vec<(PlayerCard, BaseCard)>,
}

impl Party {
    pub fn new(discord_id: i64, initial_cards: Vec<(PlayerCard, BaseCard)>) -> Party {
        let mut party = Party {
            id: discord_id,
            cards: initial_cards,
            power: 0,
        };
        party.calculate_party_power();
        party
    }

    pub fn from_entity(entity: PartyEntity, cards: Vec<(PlayerCard, BaseCard)>) -> Party {
        Party {
            id: entity.id,
            power: entity.power,
            cards,
        }
    }

    pub fn swap(&mut self, b_card_in: BaseCard, b_card_out: BaseCard, player: Player, conn: &MySqlPooledConnection) -> SakataResult<()> {
        let mut cards_out = player_card::dao::find_by(&player, &b_card_out, conn)?;
        cards_out.sort_by_key(|c| c.rarity);
        let card_out = cards_out.last().unwrap();

        let mut cards_in = player_card::dao::find_by(&player, &b_card_in, conn)?;
        cards_in.sort_by_key(|c| c.rarity);
        let card_in = cards_in.last().unwrap();

        let bp_card_in = (card_in.clone(), b_card_in);

        if self.cards.contains(&bp_card_in) {
            return Err(SakataError::BadRequest("This card is already in the party".to_string()));
        }

        for (i, c) in self.cards.iter().enumerate() {
            if &c.0 == card_out {
                self.cards.remove(i);
                break;
            }
        }

        self.cards.push(bp_card_in);
        self.calculate_party_power();
        Ok(())
    }

    fn calculate_party_power(&mut self) {
        let ov_power_sum = self.cards.iter()
            .fold(0, |sum, (pc, bc)| {
                sum + bc.overall_power as u16 + pc.rarity.get_bonus()
            });

        self.power = ov_power_sum;
    }
}