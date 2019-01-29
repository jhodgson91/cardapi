use super::cards::*;
use super::common::*;
use super::models;

use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug)]
pub enum CollectionType {
    Deck,
    Pile(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: String,

    deck: CardCollection,
    piles: HashMap<String, RefCell<CardCollection>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: Game::new_id(),
            deck: CardCollection::from(CardSelection::All(true)),
            piles: HashMap::new(),
        }
    }

    pub fn draw<'a>(
        &mut self,
        from: CollectionType,
        to: CollectionType,
        selection: &CardSelection,
    ) -> Result<(), CardAPIError> {
        match (from, to) {
            (CollectionType::Deck, CollectionType::Pile(s)) => {
                let mut p = self
                    .piles
                    .get(&s)
                    .ok_or(CardAPIError::NotFound)?
                    .borrow_mut();

                self.deck.draw(selection, &mut p);
            }
            (CollectionType::Pile(s), CollectionType::Deck) => {
                let mut p = self
                    .piles
                    .get(&s)
                    .ok_or(CardAPIError::NotFound)?
                    .borrow_mut();
                p.draw(selection, &mut self.deck);
            }
            (CollectionType::Pile(s), CollectionType::Pile(t)) => {
                let mut p1 = self
                    .piles
                    .get(&s)
                    .ok_or(CardAPIError::NotFound)?
                    .borrow_mut();
                let mut p2 = self
                    .piles
                    .get(&t)
                    .ok_or(CardAPIError::NotFound)?
                    .borrow_mut();

                p1.draw(selection, &mut p2);
            }
            _ => return Err(CardAPIError::NotFound),
        };

        Ok(())
    }

    pub fn deck(&self) -> &CardCollection {
        &self.deck
    }

    pub fn new_pile(&mut self, name: String) {
        self.piles.insert(name, RefCell::new(CardCollection::new()));
    }

    pub fn get_pile(&self, name: &String) -> Option<&RefCell<CardCollection>> {
        self.piles.get(name)
    }

    pub fn has_pile(&self, name: &String) -> bool {
        self.get_pile(name).is_some()
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
