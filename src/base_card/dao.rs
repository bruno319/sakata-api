use diesel::MysqlConnection;
use diesel::result::Error;

use crate::base_card::BaseCard;

pub struct BaseCardDao;

impl BaseCardDao {
    pub fn list(conn: &MysqlConnection) -> Result<Vec<BaseCard>, Error> {
        use crate::schema::base_cards::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        base_cards.limit(10)
            .load::<BaseCard>(conn)
    }

    pub fn insert<'a, 'b>(conn: &'b MysqlConnection, base_card: &'a BaseCard) -> Result<&'a BaseCard, String> {
        use crate::schema::base_cards;
        use diesel::RunQueryDsl;

        diesel::insert_into(base_cards::table)
            .values(base_card)
            .execute(conn)
            .map_err(|e| format!("{}", e))
            .map(|_| base_card)
    }
}