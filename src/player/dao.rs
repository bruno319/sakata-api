use diesel::{MysqlConnection, RunQueryDsl, QueryDsl};
use diesel::result::Error;
use crate::player::Player;

pub fn insert<'a, 'b>(conn: &'b MysqlConnection, player: &'a Player) -> Result<&'a Player, Error> {
    use crate::schema::players;

    diesel::insert_into(players::table)
        .values(player)
        .execute(conn)
        .map(|_| player)
}

pub fn find_by_id(conn: &MysqlConnection, id: i32) -> Result<Player, Error> {
    use crate::schema::players;

    players::table
        .find(id)
        .first(conn)
}