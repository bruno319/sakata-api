use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use super::BaseCard;

pub fn list(conn: &MysqlConnection) -> Result<Vec<BaseCard>, Error> {
    use crate::schema::base_cards::dsl::*;

    base_cards.limit(10)
        .offset(1)
        .load::<BaseCard>(conn)
}

pub fn list_by_overall_between((min, max): (i32, i32), conn: &MysqlConnection) -> Result<Vec<Option<i32>>, Error> {
    use crate::schema::base_cards::dsl::*;

    base_cards.select(id)
        .filter(overall_power.between((min - 1) as i8, (max + 1) as i8))
        .load::<Option<i32>>(conn)
}

pub fn find_by_id(conn: &MysqlConnection, id: i32) -> Result<BaseCard, Error> {
    use crate::schema::base_cards;

    base_cards::table
        .find(id)
        .first(conn)
}

pub fn save<'a, 'b>(conn: &'b MysqlConnection, base_card: &'a BaseCard) -> Result<&'a BaseCard, Error> {
    use crate::schema::base_cards;

    diesel::insert_into(base_cards::table)
        .values(base_card)
        .execute(conn)
        .map(|_| base_card)
}

