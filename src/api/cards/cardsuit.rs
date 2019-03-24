use super::*;

use serde::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardSuit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

impl HasStringCode for CardSuit {
    fn from_str(s: String) -> Option<CardSuit> {
        match s.as_str() {
            "H" => Some(CardSuit::Hearts),
            "C" => Some(CardSuit::Clubs),
            "S" => Some(CardSuit::Spades),
            "D" => Some(CardSuit::Diamonds),
            _ => None,
        }
    }

    fn to_str(&self) -> String {
        let s = match self {
            CardSuit::Hearts => "H",
            CardSuit::Clubs => "C",
            CardSuit::Spades => "S",
            CardSuit::Diamonds => "D",
        };
        s.to_string()
    }
}

impl Serialize for CardSuit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str().as_str())
    }
}

impl<'de> Deserialize<'de> for CardSuit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(super::CodeVisitor::<Self>::new())
    }
}
