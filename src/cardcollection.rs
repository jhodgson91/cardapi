use super::cardselection::*;

use diesel::prelude::*;

use super::card::Card;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardCollection {
    _cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection {
            _cards: Vec::new()
        }
    }
    pub fn from_selection(selection: CardSelection) -> CardCollection {
        CardCollection {
            _cards: CardSelection::from_all(selection),
        }
    }
    pub fn select(&self, selection: CardSelection) -> Vec<Card> {
        CardSelection::filter_cards(&self._cards, selection)
    }
    pub fn draw(&mut self, selection: CardSelection, into: &mut CardCollection) {
        let mut selected = self.select(selection);
        self.remove(&selected);
        into.add(selected);
    }
    pub fn remaining(&self) -> usize {
        self._cards.len()
    }
    pub fn contains(&self, c: &Card) -> bool {
        self._cards.contains(c)
    }
    pub fn add(&mut self, cards: Vec<Card>) {
        self.remove(&cards);
        self._cards = self._cards.iter().chain(cards.iter()).map(|c| c.to_owned()).collect();
    }

    fn remove(&mut self, cards: &Vec<Card>) {
        self._cards = self
            ._cards
            .iter()
            .filter(|c| !cards.contains(c))
            .cloned()
            .collect()
    }
}
