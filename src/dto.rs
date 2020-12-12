use serde::{Deserialize, Serialize};

use crate::types::{Class, Domain};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseCardDto {
    pub name: String,
    pub class: Class,
    pub domain: Domain,
    pub mal_id: i32,
    pub overall_power: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerDto {
    pub nickname: String,
    pub discord_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeIdsDto {
    pub anime_mal_ids: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapPartyCards {
    pub card_in: u32,
    pub card_out: u32,
}