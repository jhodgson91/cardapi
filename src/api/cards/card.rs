use super::*;

use serde::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl HasStringCode for Card {
    fn to_str(&self) -> String {
        format!("{}{}", self.value.to_str(), self.suit.to_str())
    }

    fn from_str(code: String) -> Option<Card> {
        if code.len() > 2 {
            None
        } else {
            Some(Card {
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

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(super::CodeVisitor::<Self>::new())
    }
}
