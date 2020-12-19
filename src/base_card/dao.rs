use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::dbconfig::MySqlPooledConnection;
use crate::SakataResult;

use super::BaseCard;

pub fn save<'a, 'b>(conn: &'b MySqlPooledConnection, base_card: &'a BaseCard) -> SakataResult<&'a BaseCard> {
    use crate::schema::base_cards;

    diesel::insert_into(base_cards::table)
        .values(base_card)
        .execute(conn)?;

    Ok(base_card)
}

pub fn list(conn: &MySqlPooledConnection) -> SakataResult<Vec<BaseCard>> {
    use crate::schema::base_cards::dsl::*;

    let cards = base_cards.limit(10)
        .offset(0)
        .load(conn)?;

    Ok(cards)
}

pub fn list_by_overall_between(conn: &MySqlPooledConnection, (min, max): (u8, u8)) -> SakataResult<Vec<Option<u32>>> {
    use crate::schema::base_cards::dsl::*;

    let cards = base_cards.select(id)
        .filter(overall_power.between(min - 1, max + 1))
        .load(conn)?;

    Ok(cards)
}

pub fn find_by_id(conn: &MySqlPooledConnection, id: u32) -> SakataResult<BaseCard> {
    use crate::schema::base_cards;

    let card = base_cards::table
        .find(id)
        .first(conn)?;

    Ok(card)
}

pub fn find_by_mal_id(conn: &MySqlPooledConnection, id: i32) -> SakataResult<BaseCard> {
    use crate::schema::base_cards::dsl::{mal_id, base_cards};

    let card = base_cards
        .filter(mal_id.eq(id))
        .first(conn)?;

    Ok(card)
}

pub fn verify_inserted(conn: &MySqlPooledConnection, mal_id_list: Vec<i32>) -> SakataResult<Vec<i32>> {
    use crate::schema::base_cards::dsl::{base_cards, mal_id};

    if mal_id_list.is_empty() {
        return Ok(Vec::with_capacity(0));
    }

    let result = base_cards
        .select(mal_id)
        .filter(mal_id.eq_any(mal_id_list))
        .load(conn)?;

    Ok(result)
}