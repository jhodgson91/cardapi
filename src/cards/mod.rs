mod card;
pub use card::Card;

mod cardcollection;
pub use cardcollection::CardCollection;

mod cardselection;
pub use cardselection::CardSelection;

mod cardsuit;
pub use cardsuit::CardSuit;

mod cardvalue;
pub use cardvalue::CardValue;

use super::common::*;
use super::stringcode::*;

const CARD_CODES: &'static [&str] = &[
    "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS", "AD", "2D", "3D",
    "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD", "AC", "2C", "3C", "4C", "5C", "6C",
    "7C", "8C", "9C", "0C", "JC", "QC", "KC", "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H",
    "0H", "JH", "QH", "KH",
];

pub const ALL_CARDS: &'static Fn() -> Vec<Card> = &|| {
    CARD_CODES
        .iter()
        .cloned()
        .map(|code| Card::from_str(code.to_string()).unwrap())
        .collect()
};
