use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{player_card, SakataResult};
use crate::dbconfig::MySqlPooledConnection;
use crate::party::{Party, PartyEntity};

pub fn save<'a, 'b>(conn: &'b MySqlPooledConnection, party: &'a Party) -> SakataResult<&'a Party> {
    use crate::schema::party;

    diesel::insert_into(party::table)
        .values(PartyEntity::from(party))
        .execute(conn)?;

    Ok(party)
}

pub fn find_by_discord_id(conn: &MySqlPooledConnection, discord_id: i64) -> SakataResult<Party> {
    use crate::schema::party;
    use crate::schema::party::columns::id;

    let ent: PartyEntity = party::table
        .filter(id.eq(discord_id))
        .first(conn)?;

    let card_ids = vec![
        ent.card_1.clone(),
        ent.card_2.clone(),
        ent.card_3.clone(),
        ent.card_4.clone(),
        ent.card_5.clone()
    ];
    let party_cards = player_card::dao::filter_by_list_id(card_ids, conn)?;

    Ok(Party::from_entity(ent, party_cards))
}
