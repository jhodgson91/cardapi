use super::cards::*;
use super::common::*;
use super::models;

use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug)]
pub enum CollectionType {
    Deck,
    Pile(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: String,

    deck: CardCollection,
    piles: HashMap<String, CardCollection>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: Game::new_id(),
            deck: CardCollection::from(CardSelection::All(true)),
            piles: HashMap::new(),
        }
    }

    pub fn draw(
        &mut self,
        from: CollectionType,
        to: CollectionType,
        selection: &CardSelection,
    ) -> Result<(), CardAPIError> {
        let collections = match (from, to) {
            (CollectionType::Deck, CollectionType::Pile(s)) => (
                &mut self.deck,
                self.piles.get_mut(&s).ok_or(CardAPIError::NotFound)?,
            ),
            (CollectionType::Pile(s), CollectionType::Deck) => (
                self.piles.get_mut(&s).ok_or(CardAPIError::NotFound)?,
                &mut self.deck,
            ),
            _ => return Err(CardAPIError::NotFound),
        };

        collections.0.draw(selection, collections.1)?;

        Ok(())
    }

    pub fn deck(&self) -> &CardCollection {
        &self.deck
    }

    pub fn new_pile(&mut self, name: String) {
        self.piles.insert(name, CardCollection::new());
    }

    pub fn get_pile(&self, name: &String) -> Option<&CardCollection> {
        self.piles.get(name)
    }

    pub fn has_pile(&self, name: &String) -> bool {
        self.get_pile(name).is_some()
    }

    fn verify_move(
        &self,
        from: &CollectionType,
        to: &CollectionType,
        selection: &CardSelection,
    ) -> bool {
        if let (Some(from_collection), Some(to_collection)) =
            (self.get_collection(from), self.get_collection(to))
        {
            match selection {
                CardSelection::Empty => true,
                CardSelection::Random(n) | CardSelection::Top(n) | CardSelection::Bottom(n) => {
                    from_collection.remaining() >= *n
                }
                CardSelection::Filter { suits, values } => {
                    let cards = CardCollection::from(selection.clone());
                    from_collection.contains_all(&cards) && to_collection.contains_none(&cards)
                }
                CardSelection::Cards(cards) => {
                    from_collection.contains_all(cards) && to_collection.contains_none(cards)
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn get_collection(&self, collection: &CollectionType) -> Option<&CardCollection> {
        match collection {
            CollectionType::Deck => Some(&self.deck),
            CollectionType::Pile(name) => self.piles.get(name),
        }
    }

    fn get_collection_mut(&mut self, collection: &CollectionType) -> Option<&mut CardCollection> {
        match collection {
            CollectionType::Deck => Some(&mut self.deck),
            CollectionType::Pile(name) => self.piles.get_mut(name),
        }
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

use rocket_contrib::json::JsonValue;

impl std::convert::Into<JsonValue> for Game {
    fn into(self) -> JsonValue {
        JsonValue::from(serde_json::to_value(self).unwrap())
    }
}

impl models::HasModel for Game {
    type Model = models::Game;

    fn from_model(m: Self::Model) -> Self {
        serde_json::from_str::<Game>(&m.json).unwrap()
    }
    fn to_model(&self) -> Self::Model {
        models::Game {
            id: self.id.clone(),
            json: serde_json::to_string(self).unwrap(),
        }
    }

    fn save(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use super::schema::games::dsl::*;
        use diesel::dsl::*;

        if select(exists(games.find(self.id.clone()))).get_result(conn)? {
            update(games.find(self.id.clone()))
                .set(self.to_model())
                .execute(conn)
        } else {
            insert_into(games).values(self.to_model()).execute(conn)?;
            Ok(1)
        }
    }

    fn load(conn: &SqliteConnection, id: String) -> QueryResult<Self> {
        use super::schema::games::dsl::games;

        Ok(Self::from_model(
            games.find(id).get_result::<models::Game>(conn)?,
        ))
    }
}
