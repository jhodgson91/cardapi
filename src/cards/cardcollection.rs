use super::cardselection::*;

use super::card::Card;
use std::slice::Iter;

use rocket::http::{Status, RawStr};
use rocket::request::{Request, FromFormValue, Outcome};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardCollection {
    pub(super) _cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection { _cards: Vec::new() }
    }
    pub fn select(&self, selection: CardSelection) -> Vec<Card> {
        CardSelection::filter_cards(&self._cards, selection)
    }
    pub fn remaining(&self) -> usize {
        self._cards.len()
    }
    pub fn contains(&self, c: &Card) -> bool {
        self._cards.contains(c)
    }
    pub fn add(&mut self, cards: CardCollection) {
        self.remove(&cards);
        self._cards = self
            ._cards
            .iter()
            .chain(cards.iter())
            .map(|c| c.to_owned())
            .collect();
    }

    pub fn remove(&mut self, cards: &CardCollection) {
        self._cards = self
            ._cards
            .iter()
            .filter(|c| !cards.contains(c))
            .cloned()
            .collect()
    }

    pub fn iter(&self) -> Iter<Card> {
        self._cards.iter()
    }
}

impl From<CardSelection> for CardCollection {
    fn from(selection: CardSelection) -> Self {
        CardCollection {
            _cards: CardSelection::from_all(selection),
        }
    }
}

impl<'v> FromFormValue<'v> for CardCollection {
    type Error = super::CardAPIError;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        Ok(CardCollection::new())
    }
}