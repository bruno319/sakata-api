use serde::{Deserialize, Serialize};

use crate::model::{Class, Genre};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseCardDto {
    pub class: Class,
    pub genre: Genre,
    pub mal_id: i32,
    pub image: String,
}