use super::cardselection::*;

use diesel::prelude::*;

use super::card::Card;

pub trait CardCollection {
    fn id(&self) -> String;
    fn cards(&self) -> &Vec<Card>;
    fn cards_mut(&mut self) -> &mut Vec<Card>;
    fn select(&self, selection: CardSelection) -> Vec<Card> {
        CardSelection::filter_cards(self.cards(), selection)
    }
    fn draw(&mut self, selection: CardSelection) -> Vec<Card> {
        let selected = self.select(selection);
        self.remove(&selected);
        selected
    }
    fn add(&mut self, cards: &mut Vec<Card>) {
        self.remove(&cards);
        Vec::append(self.cards_mut(), cards)
    }
    fn remove(&mut self, cards: &Vec<Card>) {
        *self.cards_mut() = self
            .cards()
            .iter()
            .filter(|c| !cards.contains(c))
            .cloned()
            .collect()
    }
    fn remaining(&self) -> usize {
        self.cards().len()
    }
    fn has_card(&self, c: &Card) -> bool {
        self.cards().contains(c)
    }
    fn load( db: &SqliteConnection, id: &str) -> QueryResult<Self>
    where
        Self: std::marker::Sized;
    fn save(&self, db: &SqliteConnection) -> Result<(), super::common::CardAPIError>;
}
