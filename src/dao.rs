use diesel::MysqlConnection;
use diesel::result::Error;

use crate::model::CharacterCard;

pub struct Cards;

impl Cards {
    pub fn list(connection: &MysqlConnection) -> Result<Vec<CharacterCard>, Error> {
        use crate::schema::cards::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        cards.limit(10)
            .load::<CharacterCard>(connection)
    }
}