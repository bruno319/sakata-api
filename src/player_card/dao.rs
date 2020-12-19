use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::base_card::BaseCard;
use crate::dbconfig::MySqlPooledConnection;
use crate::player::Player;
use crate::player_card::PlayerCard;
use crate::SakataResult;
use crate::types::model::{Class, Domain};

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

pub fn filter_by_list_id(id_list: Vec<String>, conn: &MySqlPooledConnection) -> SakataResult<Vec<(PlayerCard, BaseCard)>> {
    // "SELECT * FROM player_cards INNER JOIN base_cards WHERE id IN {}";
    use crate::schema::player_cards::dsl::player_cards;
    use crate::schema::player_cards::columns::id;
    use crate::schema::base_cards;

    let cards = player_cards
        .inner_join(base_cards::table)
        .filter(id.eq_any(id_list))
        .load(conn)?;

    Ok(cards)
}

pub fn query(player_id: u32, page: u16, class: Option<Class>, domain: Option<Domain>, conn: &MySqlPooledConnection) -> SakataResult<Vec<(PlayerCard, BaseCard)>> {
    use crate::schema::player_cards;
    use crate::schema::player_cards::player_id as p_id;
    use crate::schema::base_cards;

    let mut card_query = player_cards::dsl::player_cards
        .inner_join(base_cards::table)
        .filter(p_id.eq(player_id))
        .order_by(player_cards::overall_power.desc())
        .limit(10)
        .offset(((page - 1) * 10).into())
        .into_boxed();

    if let Some(c) = class {
        card_query = card_query.filter(base_cards::class.eq(c as i8));
    }

    if let Some(d) = domain {
        card_query = card_query.filter(base_cards::domain.eq(d as i8));
    }

    let cards = card_query.load(conn)?;

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

pub fn remove_by_id(p_id: String, conn: &MySqlPooledConnection) -> SakataResult<()> {
    use crate::schema::player_cards::dsl::*;

    diesel::delete(player_cards.filter(id.eq(p_id)))
        .execute(conn)?;

    Ok(())
}
