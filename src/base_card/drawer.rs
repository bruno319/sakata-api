use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

use crate::{base_card, SakataResult};
use crate::dbconfig::MySqlPooledConnection;

#[derive(Debug, Default, Clone)]
pub struct BaseCardDrawer {
    cards_95plus: Vec<u32>,
    cards_90plus: Vec<u32>,
    cards_85plus: Vec<u32>,
    cards_80plus: Vec<u32>,
    remaining_cards: Vec<u32>,
}

impl BaseCardDrawer {
    pub fn new(conn: &MySqlPooledConnection) -> SakataResult<BaseCardDrawer> {
        let card_map = base_card::dao::list_id_and_overall(conn)?;

        let mut drawer = BaseCardDrawer {
            cards_95plus: vec![],
            cards_90plus: vec![],
            cards_85plus: vec![],
            cards_80plus: vec![],
            remaining_cards: vec![],
        };

        for (id, overall) in card_map.into_iter() {
            match overall {
                95..=99 => drawer.cards_95plus.push(id.unwrap()),
                90..=94 => drawer.cards_90plus.push(id.unwrap()),
                85..=89 => drawer.cards_85plus.push(id.unwrap()),
                80..=84 => drawer.cards_80plus.push(id.unwrap()),
                _ => drawer.remaining_cards.push(id.unwrap()),
            }
        };

        Ok(drawer)
    }

    pub fn common_card(&self) -> u32 {
        let rand = thread_rng().gen_range(0, 100);
        let vec_to_choose = if rand < 1 {
            &self.cards_95plus
        } else if rand < 3 {
            &self.cards_90plus
        } else if rand < 7 {
            &self.cards_85plus
        } else if rand < 14 {
            &self.cards_80plus
        } else {
            &self.remaining_cards
        };

        vec_to_choose.choose(&mut thread_rng())
            .unwrap()
            .clone()
    }

    pub fn star_card(&self) -> u32 {
        let rand = thread_rng().gen_range(0, 100);
        let vec_to_choose = if rand < 25 {
            &self.cards_95plus
        } else {
            &self.cards_90plus
        };

        vec_to_choose.choose(&mut thread_rng())
            .unwrap()
            .clone()
    }
}