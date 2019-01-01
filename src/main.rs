#![feature(uniform_paths)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde::*;
use serde::de::*;

mod cardsuit;
mod cardvalue;
mod common {
    pub trait HasStringCode {
        fn from_str(s: String) -> Option<Self>
        where
            Self: std::marker::Sized;
        fn to_str(&self) -> String;
    }

    pub const CARD_CODES: &'static [&str] = &[
        "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS", "AD", "2D",
        "3D", "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD", "AC", "2C", "3C", "4C",
        "5C", "6C", "7C", "8C", "9C", "0C", "JC", "QC", "KC", "AH", "2H", "3H", "4H", "5H", "6H",
        "7H", "8H", "9H", "0H", "JH", "QH", "KH",
    ];
}

use cardsuit::CardSuit;
use cardvalue::CardValue;
use common::HasStringCode;

struct Card {
    suit: CardSuit,
    value: CardValue,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl HasStringCode for Card {
    fn to_str(&self) -> String {
        format!("{}{}", self.value.to_str(), self.suit.to_str())
    }

    fn from_str(code: String) -> Option<Card> {
        match code.len() > 2 {
            true => None,
            false => Some(Card {
                value: CardValue::from_str(code.chars().nth(0)?.to_string())?,
                suit: CardSuit::from_str(code.chars().nth(1)?.to_string())?,
            })
        }
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}{}", self.value.to_str(), self.suit.to_str()).as_str())
    }
}

struct CardVisitor;
impl<'de> Visitor<'de> for CardVisitor {
    type Value = Card;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a card code")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where 
        E: de::Error,
    {
        match Card::from_str(value.to_string()) {
            Some(card) => Ok(card),
            None => Err(E::custom("Invalid card code"))
        }
    }

}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CardVisitor)
    }
}

#[derive(Serialize,Deserialize)]
struct Pile {
    name: String,
    cards: Vec<Card>,
}

#[derive(Serialize,Deserialize)]
struct Deck {
    pub id: String,
    cards: Vec<Card>,
    piles: Vec<Pile>,
}

fn main() {
    
}
