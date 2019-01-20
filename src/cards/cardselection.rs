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

use rocket_contrib::json::{JsonError, JsonValue};
use std::fmt::Display;
use std::io::{Error, ErrorKind};

impl super::HasJsonValue for CardSelection {
    fn from_json(json: JsonValue) -> Result<Self, JsonError<'static>> {
        if let Some(o) = json.as_object() {
            let valid_keys = (
                o.get("suits"),
                o.get("values"),
                o.get("random"),
                o.get("top"),
                o.get("bottom"),
            );

            let selection = match valid_keys {
                (Some(suits), Some(values), None, None, None) => CardSelection::Filter {
                    suits: StringCodes::from_json(suits.clone()).unwrap(),
                    values: StringCodes::from_json(values.clone()).unwrap(),
                },
                (Some(suits), None, None, None, None) => CardSelection::Filter {
                    suits: StringCodes::from_json(suits.clone()).unwrap(),
                    values: StringCodes::new(),
                },
                (None, Some(values), None, None, None) => CardSelection::Filter {
                    suits: StringCodes::new(),
                    values: StringCodes::from_json(values.clone()).unwrap(),
                },
                (None, None, Some(count), None, None) => {
                    CardSelection::Random(count.as_u64().unwrap() as usize)
                }
                (None, None, None, Some(count), None) => {
                    CardSelection::Top(count.as_u64().unwrap() as usize)
                }
                (None, None, None, None, Some(count)) => {
                    CardSelection::Bottom(count.as_u64().unwrap() as usize)
                }
                _ => CardSelection::Empty,
            };
            Ok(selection)
        } else {
            Err(JsonError::Io(Error::new(
                ErrorKind::InvalidInput,
                "Invalid input to CardSelection::to_json",
            )))
        }
    }

    fn to_json(&self) -> JsonValue {
        json!({})
    }
}

use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, Status};
use rocket::{Data, Outcome, Outcome::*, Request};
use std::io::Read;

/*
impl FromDataSimple for CardSelection {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, String> {
        let content_type = ContentType::JSON;
        if (req.content_type() != Some(&content_type)) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let json = json!({ "value": string });

    }
}
*/
