use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::player::Player;
use crate::SakataResult;

pub fn save<'a, 'b>(conn: &'b MysqlConnection, player: &'a Player) -> SakataResult<&'a Player> {
    use crate::schema::players;

    diesel::insert_into(players::table)
        .values(player)
        .execute(conn)?;

    Ok(player)
}

pub fn find_by_id(conn: &MysqlConnection, id: i32) -> SakataResult<Player> {
    use crate::schema::players;

    let player = players::table
        .find(id)
        .first(conn)?;

    Ok(player)
}

pub fn find_by_discord_id(conn: &MysqlConnection, id: i64) -> SakataResult<Player> {
    use crate::schema::players;
    use crate::schema::players::dsl::discord_id;

    let player = players::table
        .filter(discord_id.eq(id))
        .first(conn)?;

    Ok(player)
}

pub fn update_coins<'a, 'b>(player: &'a Player, conn: &'b MysqlConnection) -> SakataResult<&'a Player> {
    use crate::schema::players::columns::coins;

    diesel::update(player)
        .set(coins.eq(player.coins))
        .execute(conn)?;

    Ok(player)
}

pub fn update_stardust<'a, 'b>(player: &'a Player, conn: &'b MysqlConnection) -> SakataResult<&'a Player> {
    use crate::schema::players::columns::stardust;

    diesel::update(player)
        .set(stardust.eq(player.stardust))
        .execute(conn)?;

    Ok(player)
}