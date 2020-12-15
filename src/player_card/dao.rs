use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::base_card::BaseCard;
use crate::dbconfig::MySqlPooledConnection;
use crate::player::Player;
use crate::player_card::PlayerCard;
use crate::SakataResult;

pub fn save<'a, 'b>(player_card: &'a PlayerCard, conn: &'b MySqlPooledConnection) -> SakataResult<&'a PlayerCard> {
    use crate::schema::player_cards;

    diesel::insert_into(player_cards::table)
        .values(player_card)
        .execute(conn)?;

    Ok(player_card)
}

pub fn find_by(player: &Player, base_card: &BaseCard, conn: &MySqlPooledConnection) -> SakataResult<Vec<PlayerCard>> {
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::base_card_id;
    use crate::schema::player_cards::columns::player_id;

    let cards = player_cards
        .filter(base_card_id.eq(base_card.id.unwrap()).and(player_id.eq(player.id.unwrap())))
        .limit(4)
        .load(conn)?;

    Ok(cards)
}

pub fn filter_by_list_id(ids: Vec<String>, conn: &MysqlConnection) -> SakataResult<Vec<(PlayerCard, BaseCard)>> {
    // "SELECT * FROM player_cards INNER JOIN base_cards WHERE id IN {}";
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::id;
    use crate::schema::base_cards;

    let cards = player_cards
        .inner_join(base_cards::table)
        .filter(id.eq_any(ids))
        .load(conn)?;

    Ok(cards)
}

pub fn update_quantity<'a, 'b>(player_card: &'a PlayerCard, conn: &'b MySqlPooledConnection) -> SakataResult<&'a PlayerCard> {
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::quantity;

    diesel::update(player_cards.find(player_card.id.clone()))
        .set(quantity.eq(player_card.quantity))
        .execute(conn)?;

    Ok(player_card)
}
