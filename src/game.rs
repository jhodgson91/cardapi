use super::*;

use std::collections::HashMap;

pub enum CollectionType {
    Deck,
    Pile(String),
}
#[derive(Debug)]
pub struct Game {
    _id: String,
    _deck: CardCollection,
    _piles: HashMap<String, CardCollection>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            _id: Game::new_id(),
            _deck: CardCollection::from(CardSelection::All(true)),
            _piles: HashMap::new(),
        }
    }

    pub fn new_pile(&mut self, name: String) {
        self._piles.insert(name, CardCollection::new());
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
                let mut fromCollection = match from {
                    CollectionType::Deck => &mut self._deck,
                    CollectionType::Pile(name) => self
                        ._piles
                        .get_mut(&name)
                        .ok_or(common::CardAPIError::NotFound)?,
                };
                fromCollection.remove(&cards);
            }
            {
                let mut toCollection = match to {
                    CollectionType::Deck => &mut self._deck,
                    CollectionType::Pile(name) => self
                        ._piles
                        .get_mut(&name)
                        .ok_or(common::CardAPIError::NotFound)?,
                };
                toCollection.add(cards);
            }
        }

        Ok(())
    }

    pub fn has_collection(&self, collection: &CollectionType) -> bool {
        match collection {
            CollectionType::Deck => true,
            CollectionType::Pile(name) => self._piles.get(name).is_some(),
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
