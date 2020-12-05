use serde::{Deserialize, Serialize};

use crate::model::{Class, Genre};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseCardDto {
    pub name: String,
    pub class: Class,
    pub genre: Genre,
    pub mal_id: i32,
    pub overall_power: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerDto {
    pub nickname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeIdsDto {
    pub anime_mal_ids: Vec<u32>,
}