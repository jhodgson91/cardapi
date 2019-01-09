use super::common::HasStringCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl HasStringCode for CardValue {

    fn to_str(&self) -> String {
        let s = match self {
            CardValue::Ace => "A",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "0",
            CardValue::Jack => "J",
            CardValue::Queen => "Q",
            CardValue::King => "K"
        };
        s.to_string()
    }

    fn from_str(s: String) -> Option<CardValue> {
        match s.as_str() {
            "A" => Some(CardValue::Ace),
            "2" => Some(CardValue::Two),
            "3" => Some(CardValue::Three),
            "4" => Some(CardValue::Four),
            "5" => Some(CardValue::Five),
            "6" => Some(CardValue::Six),
            "7" => Some(CardValue::Seven),
            "8" => Some(CardValue::Eight),
            "9" => Some(CardValue::Nine),
            "0" => Some(CardValue::Ten),
            "J" => Some(CardValue::Jack),
            "Q" => Some(CardValue::Queen),
            "K" => Some(CardValue::King),
            _ => None
        }
    }
}