use super::*;

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
