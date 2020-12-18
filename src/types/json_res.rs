use serde::{Deserialize, Serialize};

use crate::base_card::BaseCard;
use crate::party::Party;
use crate::player::Player;
use crate::player_card::PlayerCard;
use crate::types::model::{Class, Domain, Rarity};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerJoinedResponse {
    pub discord_id: i64,
    pub channel_id: i64,
    pub discriminator: u16,
    pub nickname: String,
    pub coins: i16,
    pub stardust: i16,
    pub party_power: u16,
    pub party: Vec<PlayerCardResponse>,
}

impl PlayerJoinedResponse {
    pub fn new(player: Player, party: Party) -> PlayerJoinedResponse {
        let party_power = party.power;
        let party = party.cards
            .into_iter()
            .map(|(pc, bc)| PlayerCardResponse::new(pc, bc))
            .collect();

        PlayerJoinedResponse {
            discord_id: player.discord_id,
            channel_id: player.channel_id,
            discriminator: player.discriminator,
            nickname: player.nickname,
            coins: player.coins,
            stardust: player.stardust,
            party_power,
            party,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerCardResponse {
    pub base_card_id: u32,
    pub player_card_id: String,
    pub mal_id: i32,
    pub name: String,
    pub rarity: Rarity,
    pub class: Class,
    pub domain: Domain,
    pub overall_power: u8,
    pub image: String,
    pub quantity: u8,
}

impl PlayerCardResponse {
    pub fn new(player_card: PlayerCard, base_card: BaseCard) -> PlayerCardResponse {
        let image_name = format!("sakata_{}_{}.jpeg", base_card.mal_id, player_card.rarity as i8);

        PlayerCardResponse {
            base_card_id: base_card.id.unwrap_or_default(),
            player_card_id: player_card.id,
            mal_id: base_card.mal_id,
            name: base_card.name.clone(),
            rarity: player_card.rarity,
            class: base_card.class,
            domain: base_card.domain,
            overall_power: player_card.overall_power,
            image: image_name,
            quantity: player_card.quantity,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartyResponse {
    pub id: i64,
    pub power: u16,
    pub cards: Vec<PlayerCardResponse>,
}

impl PartyResponse {
    pub fn new(party: Party) -> PartyResponse {
        let cards = party.cards
            .into_iter()
            .map(|(pc, bc)| PlayerCardResponse::new(pc, bc))
            .collect();

        PartyResponse {
            id: party.id,
            power: party.power,
            cards,
        }
    }
}