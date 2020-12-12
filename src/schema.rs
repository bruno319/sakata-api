table! {
    base_cards (id) {
        id -> Nullable<Unsigned<Integer>>,
        name -> Varchar,
        overall_power -> Unsigned<Tinyint>,
        class -> Tinyint,
        domain -> Tinyint,
        mal_id -> Integer,
    }
}

table! {
    party (id) {
        id -> Unsigned<Integer>,
        power -> Unsigned<Smallint>,
        card_1 -> Unsigned<Integer>,
        card_2 -> Unsigned<Integer>,
        card_3 -> Unsigned<Integer>,
        card_4 -> Unsigned<Integer>,
        card_5 -> Unsigned<Integer>,
    }
}

table! {
    player_cards (id) {
        id -> Nullable<Unsigned<Integer>>,
        base_card_id -> Unsigned<Integer>,
        player_id -> Unsigned<Integer>,
        rarity -> Tinyint,
        quantity -> Unsigned<Tinyint>,
    }
}

table! {
    players (id) {
        id -> Nullable<Unsigned<Integer>>,
        discord_id -> Bigint,
        nickname -> Varchar,
        coins -> Smallint,
        stardust -> Smallint,
    }
}

joinable!(party -> players (id));
joinable!(player_cards -> base_cards (base_card_id));
joinable!(player_cards -> players (player_id));

allow_tables_to_appear_in_same_query!(
    base_cards,
    party,
    player_cards,
    players,
);
