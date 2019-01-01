table! {
    decks (id) {
        id -> Text,
        cards -> Text,
    }
}

table! {
    piles (id) {
        id -> Text,
        name -> Text,
        deck_id -> Text,
        cards -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    decks,
    piles,
);
