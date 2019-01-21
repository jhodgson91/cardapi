use rand::seq::SliceRandom;
use rand::thread_rng;

use super::*;

#[derive(Debug)]
pub enum CardSelection {
    Empty,
    All(bool),
    Top(usize),
    Bottom(usize),
    Random(usize),
    // Both None is the same as Empty
    Filter {
        suits: StringCodes<CardSuit>,
        values: StringCodes<CardValue>,
    },
    Cards(Vec<Card>),
}

impl CardSelection {
    pub fn from_all(selection: CardSelection) -> Vec<Card> {
        CardSelection::filter_cards(
            &CARD_CODES
                .to_vec()
                .iter()
                .map(|card| Card::from_str(card.to_string()).unwrap())
                .collect(),
            selection,
        )
    }

    pub fn filter_cards(cards: &Vec<Card>, selection: CardSelection) -> Vec<Card> {
        match selection {
            CardSelection::Empty => Vec::new(),
            CardSelection::All(shuffled) => CardSelection::apply_all(cards, shuffled),
            CardSelection::Random(n) => CardSelection::apply_random(cards, n),
            CardSelection::Bottom(n) => CardSelection::apply_bottom(cards, n),
            CardSelection::Top(n) => CardSelection::apply_top(cards, n),
            CardSelection::Filter { suits, values } => {
                CardSelection::apply_filter(cards, suits, values)
            }
            CardSelection::Cards(cards) => cards
                .iter()
                .filter(|c| cards.contains(c))
                .cloned()
                .collect(),
        }
    }

    fn apply_all(cards: &Vec<Card>, shuffled: bool) -> Vec<Card> {
        if shuffled {
            cards
                .choose_multiple(&mut thread_rng(), cards.len())
                .cloned()
                .collect()
        } else {
            cards.to_owned()
        }
    }

    fn apply_random(cards: &Vec<Card>, n: usize) -> Vec<Card> {
        cards
            .choose_multiple(&mut thread_rng(), n)
            .cloned()
            .collect()
    }

    fn apply_top(cards: &Vec<Card>, n: usize) -> Vec<Card> {
        let start = if n > cards.len() { 0 } else { cards.len() - n };
        cards[start..].to_vec()
    }

    fn apply_bottom(cards: &Vec<Card>, n: usize) -> Vec<Card> {
        let end = std::cmp::min(n, cards.len());
        cards[..end].to_vec()
    }

    fn apply_filter(
        cards: &Vec<Card>,
        suits: StringCodes<CardSuit>,
        values: StringCodes<CardValue>,
    ) -> Vec<Card> {
        cards
            .iter()
            .filter(|c| {
                (suits.len() == 0 || suits.contains(&c.suit))
                    && (values.len() == 0 || values.contains(&c.value))
            })
            .cloned()
            .collect()
    }
}

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

use serde_json::json;
use serde_json::Value;
use std::fmt::Display;

impl super::HasJsonValue for CardSelection {
    fn from_json(json: Value) -> Option<Self> {
        if let Value::Object(o) = json {
            let valid_keys = (
                o.get("suits"),
                o.get("values"),
                o.get("random"),
                o.get("top"),
                o.get("bottom"),
            );

            let selection = match valid_keys {
                (Some(suits), Some(values), None, None, None) => CardSelection::Filter {
                    suits: serde_json::from_value(suits.clone()).ok()?,
                    values: serde_json::from_value(values.clone()).ok()?,
                },
                (Some(suits), None, None, None, None) => CardSelection::Filter {
                    suits: serde_json::from_value(suits.clone()).ok()?,
                    values: StringCodes::new(),
                },
                (None, Some(values), None, None, None) => CardSelection::Filter {
                    suits: StringCodes::new(),
                    values: serde_json::from_value(values.clone()).ok()?,
                },
                (None, None, Some(count), None, None) => {
                    CardSelection::Random(count.as_u64()? as usize)
                }
                (None, None, None, Some(count), None) => {
                    CardSelection::Top(count.as_u64()? as usize)
                }
                (None, None, None, None, Some(count)) => {
                    CardSelection::Bottom(count.as_u64()? as usize)
                }
                _ => CardSelection::Empty,
            };
            Some(selection)
        } else {
            None
        }
    }

    fn to_json(&self) -> Value {
        match self {
            CardSelection::Filter { suits, values } => json!({
                "suits": suits,
                "values": values,
            }),
            CardSelection::Random(count) => json!({ "random": count }),
            CardSelection::Bottom(count) => json!({ "bottom": count }),
            CardSelection::Top(count) => json!({ "top": count }),
            _ => json!({}),
        }
    }
}
