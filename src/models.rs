use super::schema::decks;
use super::schema::piles;

#[derive(Identifiable, Insertable, Queryable, AsChangeset, PartialEq, Debug)]
#[table_name="decks"]
pub struct DeckData {
    pub id: String,
    pub cards: String
}

#[derive(Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Deck)]
#[table_name="piles"]
pub struct PileData {
    pub id: String,
    pub deck_id: String,
    pub name: String,
    pub cards: String
}