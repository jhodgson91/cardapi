use diesel::prelude::*;

use super::common;
use super::models;
use super::schema;

use super::cardcollection::*;
use super::card::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pile {
    _name: String,
    _deck_id: String,
    _cards: Vec<Card>,
}

impl Pile {
    pub fn name(&self) -> &String {
        &self._name
    }

    pub fn new(name: String, deck_id: &String, cards: Vec<Card>) -> Pile {
        Pile {
            _name: name,
            _deck_id: deck_id.clone(),
            _cards: cards
        }
    }
}

impl CardCollection for Pile {
    fn id(&self) -> String {
        format!("{}::{}", self._deck_id, self._name)
    }

    fn cards(&self) -> &Vec<Card> {
        &self._cards
    }

    fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self._cards
    }

    fn load(db: &SqliteConnection, identifier: &str) -> QueryResult<Self> {
        use schema::piles::dsl::*;

        let data = piles.find(identifier).get_result::<models::Pile>(db)?;
        Ok(Pile {
            _name: data.name,
            _deck_id: data.deck_id,
            _cards: serde_json::from_str(data.cards.as_str()).unwrap(),
        })
    }

    fn save(&self, db: &SqliteConnection) -> Result<(), common::CardAPIError> {
        use schema::piles::dsl::*;

        let data = models::Pile {
            id: self.id(),
            name: self._name.clone(),
            deck_id: self._deck_id.clone(),
            cards: serde_json::to_string(self.cards()).expect("Pile Save - Serialization Error"),
        };

        let mut result = diesel::update(piles.find(self.id()))
            .set(&data)
            .execute(db)?;
        if result == 0 {
            result = diesel::insert_into(piles).values(&data).execute(db)?;
        }
        Ok(())
    }
}