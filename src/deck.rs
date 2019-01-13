use diesel::prelude::*;

use super::common;
use super::models;
use super::schema;

use super::cards::*;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    _id: String,
    _cards: CardCollection,
    _piles: HashMap<String, CardCollection>,
}

impl Deck {
    pub fn new(selection: CardSelection) -> Result<Deck, common::CardAPIError> {
        let result = Deck {
            _id: Deck::new_id(),
            _cards: CardCollection::from(selection),
            _piles: HashMap::new(),
        };

        Ok(result)
    }

    pub fn new_pile(&mut self, name: String) -> Result<&CardCollection, common::CardAPIError> {
        if let Some(p) = self.get_pile(&name) {
            return Err(common::CardAPIError::AlreadyExists);
        }

        self._piles.insert(name.clone(), CardCollection::new());
        Ok(&self._piles[&name])
    }

    pub fn has_pile(&self, name: &String) -> bool {
        self.get_pile(name).is_some()
    }

    pub fn get_pile(&self, name: &String) -> Option<&CardCollection> {
        self._piles.get(name)
    }

    pub fn id(&self) -> &String {
        &self._id
    }

    pub fn cards(&self) -> &CardCollection {
        &self._cards
    }

    pub fn cards_mut(&mut self) -> &mut CardCollection {
        &mut self._cards
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
