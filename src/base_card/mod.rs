use jikan_rs::client::Jikan;
use jikan_rs::prelude::{Anime, Character};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::BaseCardDto;
use crate::model::{Class, Genre};
use crate::schema::base_cards;

mod dao;
pub mod handlers;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "base_cards"]
pub struct BaseCard {
    id: String,
    name: String,
    overall_power: i8,
    class: Class,
    genre: Genre,
    mal_id: i32,
    image: String,
}

impl BaseCard {
    pub async fn new(dto: BaseCardDto) -> Result<BaseCard, String> {
        let jikan = Jikan::new();
        let character = jikan.find_character(dto.mal_id as u32)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let overall_power = calc_overall_power(&jikan, &character).await?;

        let base_card = BaseCard {
            id: Uuid::new_v4().to_string(),
            name: character.name,
            overall_power,
            class: dto.class,
            genre: dto.genre,
            mal_id: dto.mal_id,
            image: dto.image,
        };

        Ok(base_card)
    }
}

async fn calc_overall_power(jikan: &Jikan, character: &Character) -> Result<i8, String> {
    let animes = find_animes_from_character(jikan, &character).await;
    if animes.is_empty() {
        return Err(format!("No animes were found for character {}", character.name));
    }

    let (tv_series, movies): (Vec<Anime>, Vec<Anime>) = animes.into_iter()
        .filter(|a| a.anime_type == "TV" || a.anime_type == "Movie")
        .partition(|a| a.anime_type == "TV");

    for tv in &tv_series {
        info!("{}: {:?}, {:?}", tv.title, tv.score, tv.members);
    }

    let ov_fav = calc_overall_member_favorites(character.member_favorites);
    let (ov_pop, ov_scr) = if !tv_series.is_empty() {
        (calc_overall_popularity(&tv_series), calc_overall_score(&tv_series))
    } else {
        (calc_overall_popularity(&movies), calc_overall_score(&movies))
    };

    info!("{}. Favorites Overall: {}, Popularity Anime Overall: {}, Score Anime Overall: {}"
          , character.name, ov_fav, ov_pop, ov_scr);

    let ov_power = ov_fav + ov_pop + ov_scr;

    Ok(ov_power.round().min(99.0) as i8)
}

async fn find_animes_from_character(jikan: &Jikan, character: &Character) -> Vec<Anime> {
    let mut animes = vec![];
    for anime in &character.animeography {
        let anime_result = jikan.find_anime(anime.mal_id).await;
        if anime_result.is_err() {
            continue;
        }
        let anime_result = anime_result.unwrap();
        animes.push(anime_result);
    };
    animes
}

fn calc_overall_member_favorites(members: u32) -> f32 {
    match members {
        0..=1000 => (members as f32 / 200.0) + 15.0,
        1001..=5000 => (members as f32 - 1000.0) / 800.0 + 20.0,
        5001..=25000 => (members as f32 - 5000.0) / 4000.0 + 25.0,
        25001..=50000 => (members as f32 - 25000.0) / 5000.0 + 30.0,
        _ => 35.0
    }
}

fn calc_overall_popularity(animes: &Vec<Anime>) -> f32 {
    let members = animes.iter()
        .max_by_key(|a| a.members)
        .unwrap()
        .members
        .unwrap_or_default();

    match members {
        0..=100000 => members as f32 / 10000.0 + 10.0,
        100001..=500000 => (members as f32 - 100000.0) / 40000.0 + 20.0,
        500001..=1500000 => (members as f32 - 500000.0) / 200000.0 + 30.0,
        _ => 35.0
    }
}

fn calc_overall_score(animes: &Vec<Anime>) -> f32 {
    let mut animes = animes.iter()
        .filter_map(|a| a.score)
        .collect::<Vec<f32>>();

    if animes.len() > 1 {
        animes.sort_by(|a, b| b.partial_cmp(a).unwrap());
        animes = vec![animes[0], animes[1]];
    }

    let quantity = animes.len();
    let mean_score = animes.iter()
        .sum::<f32>() / quantity as f32;

    (mean_score - 5.0) / 0.11
}
