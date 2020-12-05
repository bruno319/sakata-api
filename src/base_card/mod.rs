use std::thread;
use std::time::Duration;

use diesel::MysqlConnection;
use jikan_rs::client::Jikan;
use jikan_rs::prelude::{Anime, Character};
use log::*;
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::dto::BaseCardDto;
use crate::model::{Class, Genre};
use crate::schema::base_cards;

mod dao;
pub mod handlers;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug)]
pub struct BaseCard {
    pub id: Option<i32>,
    pub name: String,
    pub overall_power: i8,
    pub class: Class,
    pub genre: Genre,
    pub mal_id: i32,
}

impl BaseCard {
    pub async fn new(dto: BaseCardDto) -> Result<BaseCard, String> {
        let base_card = BaseCard {
            id: None,
            name: dto.name,
            overall_power: dto.overall_power as i8,
            class: dto.class,
            genre: dto.genre,
            mal_id: dto.mal_id,
        };

        Ok(base_card)
    }
}

pub async fn calc_overall_power(mal_id: u32, anime_mal_ids: Vec<u32>) -> Result<i8, String> {
    let jikan = Jikan::new();
    let character = jikan.find_character(mal_id)
        .await
        .map_err(|e| format!("{:?}", e))?;

    let animes = if anime_mal_ids.is_empty() {
        find_animes_from_character(&jikan, &character).await
    } else {
        find_animes_from_mal_id(&jikan, anime_mal_ids).await
    };

    if animes.is_empty() {
        return Err(format!("No animes were found for character {}", character.name));
    }

    let (tv_series, movies): (Vec<Anime>, Vec<Anime>) = animes.into_iter()
        .filter(|a| a.anime_type == "TV" || a.anime_type == "Movie")
        .partition(|a| a.anime_type == "TV");

    let ov_fav = calc_overall_member_favorites(character.member_favorites);
    let (ov_pop, ov_scr) = if !tv_series.is_empty() {
        (calc_overall_popularity(&tv_series), calc_overall_score(&tv_series))
    } else {
        (calc_overall_popularity(&movies), calc_overall_score(&movies))
    };

    debug!("{}. Favorites Overall: {}, Popularity Anime Overall: {}, Score Anime Overall: {}"
           , character.name, ov_fav, ov_pop, ov_scr);

    let ov_power = ov_fav + ov_pop + ov_scr;

    Ok(ov_power.round().min(99.0) as i8)
}

async fn find_animes_from_character(jikan: &Jikan, character: &Character) -> Vec<Anime> {
    let mut animes = vec![];
    let request_times = character.animeography.len();
    for anime in &character.animeography {
        let anime_result = jikan.find_anime(anime.mal_id).await;
        if anime_result.is_err() {
            continue;
        }
        if request_times > 5 {
            thread::sleep(Duration::from_secs(3));
        }
        let anime_result = anime_result.unwrap();
        animes.push(anime_result);
    };
    animes
}

async fn find_animes_from_mal_id(jikan: &Jikan, mal_ids: Vec<u32>) -> Vec<Anime> {
    let mut animes = vec![];
    for id in mal_ids {
        let anime_result = jikan.find_anime(id).await;
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

pub fn common_card(conn: &MysqlConnection) -> Result<BaseCard, String> {
    let rand = thread_rng().gen_range(0, 100);
    let (min_overall, max_overall) = if rand < 4 {
        (90, 99)
    } else if rand < 16 {
        (80, 89)
    } else {
        (0, 79)
    };

    let range_cards = dao::list_by_overall_between((min_overall, max_overall), conn)
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter_map(|c| c)
        .collect::<Vec<i32>>();

    let card_id = range_cards.choose(&mut thread_rng())
        .ok_or("No card returned from database")?;

    dao::find_by_id(conn, *card_id)
        .map_err(|e| e.to_string())
}