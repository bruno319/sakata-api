use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use super::BaseCard;

pub fn list(conn: &MysqlConnection) -> Result<Vec<BaseCard>, Error> {
    use crate::schema::base_cards::dsl::*;

    base_cards.limit(10)
        .offset(1)
        .load::<BaseCard>(conn)
}

pub fn find_by_id(conn: &MysqlConnection, id: i32) -> Result<BaseCard, Error> {
    use crate::schema::base_cards;

    base_cards::table.find(id)
        .first(conn)
}

pub fn insert<'a, 'b>(conn: &'b MysqlConnection, base_card: &'a BaseCard) -> Result<&'a BaseCard, Error> {
    use crate::schema::base_cards;

    diesel::insert_into(base_cards::table)
        .values(base_card)
        .execute(conn)
        .map(|_| base_card)
}