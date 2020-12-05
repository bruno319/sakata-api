use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use crate::base_card::BaseCard;
use crate::player::Player;
use crate::player_card::PlayerCard;

pub fn save<'a, 'b>(player_card: &'a PlayerCard, conn: &'b MysqlConnection) -> Result<&'a PlayerCard, Error> {
    use crate::schema::player_cards;

    diesel::insert_into(player_cards::table)
        .values(player_card)
        .execute(conn)
        .map(|_| player_card)
}

pub fn find_by(player: &Player, base_card: &BaseCard, conn: &MysqlConnection) -> Result<Vec<PlayerCard>, Error> {
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::base_card_id;
    use crate::schema::player_cards::columns::player_id;

    player_cards
        .filter(base_card_id.eq(base_card.id.unwrap()).and(player_id.eq(player.id.unwrap())))
        .limit(4)
        .load(conn)
}

pub fn update_quantity<'a, 'b>(player_card: &'a PlayerCard, conn: &'b MysqlConnection) -> Result<&'a PlayerCard, Error> {
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::quantity;

    diesel::update(player_cards.find(player_card.id))
        .set(quantity.eq(player_card.quantity))
        .execute(conn)?;

    Ok(player_card)
}
