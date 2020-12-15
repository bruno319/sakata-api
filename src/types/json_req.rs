use serde::{Deserialize, Serialize};

use crate::types::model::{Class, Domain};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseCardJson {
    pub name: String,
    pub class: Class,
    pub domain: Domain,
    pub mal_id: i32,
    pub overall_power: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerJson {
    pub nickname: String,
    pub discord_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimesJson {
    pub animes: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapPartyCardsJson {
    pub card_in: i32,
    pub card_out: i32,
}