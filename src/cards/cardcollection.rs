use super::cardselection::*;

use super::card::Card;
use std::slice::Iter;

use rocket::http::{Status, RawStr};
use rocket::request::{Request, FromFormValue, Outcome};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardCollection {
    pub(super) cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection { cards: Vec::new() }
    }
    pub fn select(&self, selection: CardSelection) -> Vec<Card> {
        CardSelection::filter_cards(&self.cards, selection)
    }
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
    pub fn contains(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }
    pub fn add(&mut self, cards: CardCollection) {
        self.remove(&cards);
        self.cards = self
            .cards
            .iter()
            .chain(cards.iter())
            .map(|c| c.to_owned())
            .collect();
    }

    pub fn remove(&mut self, cards: &CardCollection) {
        self.cards = self
            .cards
            .iter()
            .filter(|c| !cards.contains(c))
            .cloned()
            .collect()
    }

    pub fn iter(&self) -> Iter<Card> {
        self.cards.iter()
    }
}

impl From<CardSelection> for CardCollection {
    fn from(selection: CardSelection) -> Self {
        CardCollection {
            cards: CardSelection::from_all(selection),
        }
    }
}

impl<'v> FromFormValue<'v> for CardCollection {
    type Error = super::CardAPIError;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        Ok(CardCollection::new())
    }
}