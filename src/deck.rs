use diesel::prelude::*;

use super::common;
use super::models;
use super::schema;

use super::card::*;
use super::cardcollection::*;
use super::cardselection::*;
use super::pile::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    _id: String,
    _cards: Vec<Card>,
    _piles: Vec<Pile>,
}

impl Deck {
    pub fn new(
        conn: &SqliteConnection,
        selection: CardSelection,
    ) -> Result<Deck, common::CardAPIError> {
        let result = Deck {
            _id: Deck::new_id(),
            _cards: CardSelection::from_all(selection),
            _piles: Vec::new(),
        };

        result.save(conn)?;
        Ok(result)
    }

    pub fn new_pile(
        &mut self,
        db: &SqliteConnection,
        name: String,
    ) -> Result<&Pile, common::CardAPIError> {
        if let Some(p) = self.get_pile(&name) {
            return Err(common::CardAPIError::AlreadyExists);
        }

        let result = Pile::new(name, &self.id(), Vec::new());

        result.save(db)?;
        self._piles.push(result);
        self.save(db)?;
        Ok(&self._piles[self._piles.len() - 1])
    }

    pub fn has_pile(&self, name: &String) -> bool {
        self.get_pile(name).is_some()
    }

    pub fn get_pile(&self, name: &String) -> Option<&Pile> {
        self._piles.iter().find(|p| p.name() == name)
    }

    fn new_id() -> String {
        use rand::seq::IteratorRandom;
        use rand::thread_rng;

        (0..26)
            .chain(32..58)
            .map(|x| (x + 'A' as u8) as char)
            .choose_multiple(&mut thread_rng(), 12)
            .iter()
            .collect()
    }
}

impl CardCollection for Deck {
    fn id(&self) -> String {
        self._id.clone()
    }

    fn cards(&self) -> &Vec<Card> {
        &self._cards
    }

    fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self._cards
    }

    fn load(db: &SqliteConnection, identifier: &str) -> QueryResult<Self> {
        use schema::decks::dsl::*;

        let data = decks.find(identifier).get_result::<models::Deck>(db)?;
        let children = models::Pile::belonging_to(&data).load::<models::Pile>(db)?;

        Ok(Deck {
            _id: data.id,
            _cards: serde_json::from_str(data.cards.as_str()).unwrap(),
            _piles: children
                .iter()
                .map(|p| {
                    Pile::new(
                        p.name.clone(),
                        &p.deck_id,
                        serde_json::from_str(p.cards.as_str()).unwrap(),
                    )
                })
                .collect(),
        })
    }

    fn save(&self, db: &SqliteConnection) -> Result<(), common::CardAPIError> {
        use schema::decks::dsl::*;

        let data = models::Deck {
            id: self._id.clone(),
            cards: serde_json::to_string(&self._cards).expect("Deck Save - Serialization error"),
        };

        let result = diesel::update(decks.find(self.id()))
            .set(&data)
            .execute(db)?;
        if result == 0 {
            println!("No deck found - creating new with id {}", self._id);
            diesel::insert_into(decks).values(&data).execute(db)?;
        }
        for p in &self._piles {
            p.save(db)?;
        }

        Ok(())
    }
}
