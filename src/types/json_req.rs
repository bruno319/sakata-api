use serde::Deserialize;

use crate::types::model::{Class, Domain};

#[derive(Deserialize, Debug, Clone)]
pub struct BaseCardJson {
    pub name: String,
    pub class: Class,
    pub domain: Domain,
    pub mal_id: i32,
    pub overall_power: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerJson {
    pub nickname: String,
    pub discord_id: i64,
    pub channel_id: i64,
    pub discriminator: u16,
}

#[derive(Deserialize, Debug)]
pub struct AnimesJson {
    pub animes: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct SwapPartyCardsJson {
    pub card_in: i32,
    pub card_out: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerCardQuery {
    pub class: Option<i8>,
    pub domain: Option<i8>,
    pub page: u16,
}