use rand::seq::SliceRandom;
use rand::thread_rng;

use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
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

const LIMIT: u64 = 256;

use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, Status};
use rocket::{Data, Outcome, Outcome::*, Request};
use std::io::Read;

impl FromDataSimple for CardSelection {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let person_ct = ContentType::JSON;
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        match serde_json::from_str::<CardSelection>(string.as_str()) {
            Ok(result) => Outcome::Success(result),
            Err(e) => Failure((Status::InternalServerError, format!("{:?}", e))),
        }
    }
}
