use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::base_card::BaseCard;
use crate::player::Player;
use crate::player_card::PlayerCard;
use crate::SakataResult;

pub fn save<'a, 'b>(player_card: &'a PlayerCard, conn: &'b MysqlConnection) -> SakataResult<&'a PlayerCard> {
    use crate::schema::player_cards;

    diesel::insert_into(player_cards::table)
        .values(player_card)
        .execute(conn)?;

    Ok(player_card)
}

pub fn find_by(player: &Player, base_card: &BaseCard, conn: &MysqlConnection) -> SakataResult<Vec<PlayerCard>> {
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::base_card_id;
    use crate::schema::player_cards::columns::player_id;

    let cards = player_cards
        .filter(base_card_id.eq(base_card.id.unwrap()).and(player_id.eq(player.id.unwrap())))
        .limit(4)
        .load(conn)?;

    Ok(cards)
}

pub fn update_quantity<'a, 'b>(player_card: &'a PlayerCard, conn: &'b MysqlConnection) -> SakataResult<&'a PlayerCard> {
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::quantity;

    diesel::update(player_cards.find(player_card.id))
        .set(quantity.eq(player_card.quantity))
        .execute(conn)?;

    Ok(player_card)
}
