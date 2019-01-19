use rand::seq::SliceRandom;
use rand::thread_rng;

use super::*;

pub enum CardSelection {
    Empty,
    All(bool),
    Top(usize),
    Bottom(usize),
    Random(usize),
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
        cards[start..cards.len()].to_vec()
    }

    fn apply_bottom(cards: &Vec<Card>, n: usize) -> Vec<Card> {
        let end = std::cmp::min(n, cards.len());
        cards[0..end].to_vec()
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
