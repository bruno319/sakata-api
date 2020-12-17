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
        id -> Bigint,
        power -> Unsigned<Smallint>,
        card_1 -> Varchar,
        card_2 -> Varchar,
        card_3 -> Varchar,
        card_4 -> Varchar,
        card_5 -> Varchar,
    }
}

table! {
    player_cards (id) {
        id -> Varchar,
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
        channel_id -> Bigint,
        nickname -> Varchar,
        discriminator -> Unsigned<Smallint>,
        coins -> Smallint,
        stardust -> Smallint,
    }
}

joinable!(player_cards -> base_cards (base_card_id));
joinable!(player_cards -> players (player_id));

allow_tables_to_appear_in_same_query!(
    base_cards,
    party,
    player_cards,
    players,
);
