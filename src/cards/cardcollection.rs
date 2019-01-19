use super::cardselection::*;

use super::card::Card;
use std::slice::Iter;

use rocket::http::RawStr;
use rocket::request::FromFormValue;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Debug)]
pub struct CardCollection {
    pub(super) cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection { cards: Vec::new() }
    }
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
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
