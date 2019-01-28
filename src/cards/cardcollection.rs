use super::cardselection::*;

use super::CardAPIError;
use super::{Card, CardSuit, CardValue};
use std::slice::Iter;

use rand::seq::IteratorRandom;
use rand::thread_rng;

use super::{HasStringCode, StringCodes};

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Clone, Debug)]
pub struct CardCollection {
    pub(super) cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection { cards: Vec::new() }
    }
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
    pub fn contains_none(&self, cards: &CardCollection) -> bool {
        for c in cards.iter() {
            if self.cards.contains(c) {
                return false;
            }
        }
        return true;
    }
    pub fn contains_all(&self, cards: &CardCollection) -> bool {
        for c in cards.iter() {
            if !self.cards.contains(c) {
                return false;
            }
        }
        return true;
    }
    pub fn contains(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }
    pub fn draw(
        &mut self,
        selection: CardSelection,
        into: &mut CardCollection,
    ) -> Result<(), CardAPIError> {
        Ok(())
    }

    pub fn iter(&self) -> Iter<Card> {
        self.cards.iter()
    }

    fn filter_cards(cards: &mut Vec<Card>, selection: CardSelection) -> Vec<Card> {
        match selection {
            CardSelection::Empty => Vec::new(),
            CardSelection::All(shuffled) => CardCollection::apply_all(cards, shuffled),
            CardSelection::Random(n) => CardCollection::apply_random(cards, n),
            CardSelection::Bottom(n) => CardCollection::apply_bottom(cards, n),
            CardSelection::Top(n) => CardCollection::apply_top(cards, n),
            CardSelection::Filter { suits, values } => {
                CardCollection::apply_filter(cards, suits, values)
            }
            CardSelection::Cards(collection) => {
                cards.retain(|card| !collection.cards.contains(&card));
                collection.cards
            }
        }
    }

    fn apply_all(cards: &mut Vec<Card>, shuffled: bool) -> Vec<Card> {
        let result = cards
            .iter()
            .cloned()
            .choose_multiple(&mut thread_rng(), cards.len());
        *cards = Vec::new();
        result
    }

    fn apply_random(cards: &mut Vec<Card>, num: usize) -> Vec<Card> {
        let result = cards
            .iter()
            .cloned()
            .choose_multiple(&mut thread_rng(), num);
        cards.retain(|c| !result.contains(&c));
        result
    }

    fn apply_top(cards: &mut Vec<Card>, n: usize) -> Vec<Card> {
        let start = if n > cards.len() { 0 } else { cards.len() - n };
        let result = cards[start..].to_vec();
        cards.retain(|c| !result.contains(&c));
        result
    }

    fn apply_bottom(cards: &mut Vec<Card>, n: usize) -> Vec<Card> {
        let end = std::cmp::min(n, cards.len());
        let result = cards[..end].to_vec();
        cards.retain(|c| !result.contains(&c));
        result
    }

    fn apply_filter(
        cards: &mut Vec<Card>,
        suits: StringCodes<CardSuit>,
        values: StringCodes<CardValue>,
    ) -> Vec<Card> {
        let result: Vec<Card> = cards
            .iter()
            .filter(|c| {
                (suits.len() == 0 || suits.contains(&c.suit))
                    && (values.len() == 0 || values.contains(&c.value))
            })
            .cloned()
            .collect();
        cards.retain(|c| !result.contains(&c));
        result
    }
}

impl std::ops::Index<usize> for CardCollection {
    type Output = Card;
    fn index(&self, index: usize) -> &Card {
        self.cards.index(index)
    }
}

impl Serialize for CardCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.cards.len()))?;
        for card in &self.cards {
            seq.serialize_element(card)?;
        }
        seq.end()
    }
}

struct CardCollectionVisitor;
impl<'de> Visitor<'de> for CardCollectionVisitor {
    type Value = CardCollection;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a card code")
    }
    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(CardCollection { cards: vec })
    }
}

impl<'de> Deserialize<'de> for CardCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CardCollectionVisitor)
    }
}

impl From<CardSelection> for CardCollection {
    fn from(selection: CardSelection) -> Self {
        let mut cards = super::CARD_CODES
            .to_vec()
            .iter()
            .map(|card| Card::from_str(card.to_string()).unwrap())
            .collect();
        CardCollection {
            cards: CardCollection::filter_cards(&mut cards, selection),
        }
    }
}
