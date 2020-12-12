use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::SakataResult;

use super::BaseCard;

pub fn list(conn: &MysqlConnection) -> SakataResult<Vec<BaseCard>> {
    use crate::schema::base_cards::dsl::*;

    let cards = base_cards.limit(10)
        .offset(0)
        .load(conn)?;

    Ok(cards)
}

pub fn list_by_overall_between((min, max): (u8, u8), conn: &MysqlConnection) -> SakataResult<Vec<Option<u32>>> {
    use crate::schema::base_cards::dsl::*;

    let cards = base_cards.select(id)
        .filter(overall_power.between(min - 1, max + 1))
        .load(conn)?;

    Ok(cards)
}

pub fn find_by_id(conn: &MysqlConnection, id: u32) -> SakataResult<BaseCard> {
    use crate::schema::base_cards;

    let card = base_cards::table
        .find(id)
        .first(conn)?;

    Ok(card)
}

pub fn save<'a, 'b>(conn: &'b MysqlConnection, base_card: &'a BaseCard) -> SakataResult<&'a BaseCard> {
    use crate::schema::base_cards;

    diesel::insert_into(base_cards::table)
        .values(base_card)
        .execute(conn)?;

    Ok(base_card)
}

