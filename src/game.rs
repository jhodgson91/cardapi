use super::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

    pub fn new_pile(&mut self, name: String) {
        self.piles.insert(name, CardCollection::new());
    }

    pub fn move_cards(
        &mut self,
        from: CollectionType,
        to: CollectionType,
        selection: CardSelection,
    ) -> Result<(), common::CardAPIError> {
        if self.has_collection(&from) && self.has_collection(&to) {
            let cards = CardCollection::from(selection);
            {
                let mut from_collection = match from {
                    CollectionType::Deck => &mut self.deck,
                    CollectionType::Pile(name) => self
                        .piles
                        .get_mut(&name)
                        .ok_or(common::CardAPIError::NotFound)?,
                };
                from_collection.remove(&cards);
            }
            {
                let mut to_collection = match to {
                    CollectionType::Deck => &mut self.deck,
                    CollectionType::Pile(name) => self
                        .piles
                        .get_mut(&name)
                        .ok_or(common::CardAPIError::NotFound)?,
                };
                to_collection.add(cards);
            }
        }

        Ok(())
    }

    fn has_collection(&self, collection: &CollectionType) -> bool {
        match collection {
            CollectionType::Deck => true,
            CollectionType::Pile(name) => self.piles.get(name).is_some(),
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
