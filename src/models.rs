use super::schema::decks;
use super::schema::piles;

use diesel::prelude::*;

#[derive(Identifiable, Insertable, Queryable, AsChangeset, PartialEq, Debug)]
#[table_name = "decks"]
pub struct Deck {
    pub id: String,
    pub cards: String,
}

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Associations, PartialEq, Debug)]
#[belongs_to(Deck)]
#[table_name = "piles"]
pub struct Pile {
    pub id: String,
    pub name: String,
    pub deck_id: String,
    pub cards: String,
}
