use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use crate::player::Player;

pub fn save<'a, 'b>(conn: &'b MysqlConnection, player: &'a Player) -> Result<&'a Player, Error> {
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

pub fn find_by_discord_id(conn: &MysqlConnection, id: i64) -> Result<Player, Error> {
    use crate::schema::players;
    use crate::schema::players::dsl::discord_id;

    players::table
        .filter(discord_id.eq(id))
        .first(conn)
}

pub fn update_coins<'a, 'b>(player: &'a Player, conn: &'b MysqlConnection) -> Result<&'a Player, Error> {
    use crate::schema::players::columns::coins;

    diesel::update(player)
        .set(coins.eq(player.coins))
        .execute(conn)?;

    Ok(player)
}