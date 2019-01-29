use super::*;

use std::slice::Iter;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Clone, Debug)]
pub struct CardCollection {
    pub(self) cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection { cards: Vec::new() }
    }
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
    pub fn contains(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }
    pub fn draw(
        &mut self,
        selection: &CardSelection,
        into: &mut CardCollection,
    ) -> Result<(), CardAPIError> {
        let mut to_draw = CardCollection::select_cards(&self.cards, selection)?;

        if self.cards.iter().filter(|c| to_draw.contains(c)).count() == to_draw.len()
            && into.cards.iter().filter(|c| to_draw.contains(c)).count() == 0
        {
            self.cards.retain(|c| !to_draw.contains(&c));
            into.cards.append(&mut to_draw);
        }

        Ok(())
    }

    fn select_cards(
        cards: &Vec<Card>,
        selection: &CardSelection,
    ) -> Result<Vec<Card>, CardAPIError> {
        match selection {
            CardSelection::Empty => Ok(Vec::new()),
            CardSelection::All(shuffled) => CardCollection::select_all(cards, shuffled),
            CardSelection::Random(n) => CardCollection::select_random(cards, n),
            CardSelection::Bottom(n) => CardCollection::select_bottom(cards, n),
            CardSelection::Top(n) => CardCollection::select_top(cards, n),
            CardSelection::Filter { suits, values } => {
                CardCollection::select_filter(cards, suits, values)
            }
            CardSelection::Cards(collection) => Ok(collection.cards.clone()),
        }
    }

    fn select_all(from: &Vec<Card>, shuffled: &bool) -> Result<Vec<Card>, CardAPIError> {
        if *shuffled {
            CardCollection::select_random(from, &from.len())
        } else {
            println!("this ran...");
            Ok(from.clone())
        }
    }

    fn select_random(from: &Vec<Card>, n: &usize) -> Result<Vec<Card>, CardAPIError> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        if *n <= from.len() {
            Ok(from
                .as_slice()
                .choose_multiple(&mut thread_rng(), *n)
                .cloned()
                .collect())
        } else {
            Err(CardAPIError::NotEnoughCards)
        }
    }

    fn select_top(from: &Vec<Card>, n: &usize) -> Result<Vec<Card>, CardAPIError> {
        if (*n <= from.len()) {
            let start = if *n > from.len() { 0 } else { from.len() - n };
            Ok(from[start..].to_vec())
        } else {
            Err(CardAPIError::NotEnoughCards)
        }
    }

    fn select_bottom(from: &Vec<Card>, n: &usize) -> Result<Vec<Card>, CardAPIError> {
        if *n < from.len() {
            let end = std::cmp::min(*n, from.len());
            Ok(from[..end].to_vec())
        } else {
            Err(CardAPIError::NotEnoughCards)
        }
    }

    fn select_filter(
        from: &Vec<Card>,
        suits: &StringCodes<CardSuit>,
        values: &StringCodes<CardValue>,
    ) -> Result<Vec<Card>, CardAPIError> {
        let test_suit = |c: &Card| suits.len() == 0 || suits.contains(&c.suit);
        let test_value = |c: &Card| values.len() == 0 || values.contains(&c.value);

        let expected = suits.len() * values.len();

        let filtered = from.iter().filter(|&c| test_suit(c) && test_value(c));

        if filtered.clone().count() >= expected {
            Ok(filtered.cloned().collect())
        } else {
            Err(CardAPIError::CardNotInCollection)
        }
    }
}

impl From<CardSelection> for CardCollection {
    fn from(selection: CardSelection) -> Self {
        CardCollection {
            cards: CardCollection::select_cards(&ALL_CARDS(), &selection).unwrap(),
        }
    }
}

impl Serialize for CardCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.cards.len()))?;
        for card in &self.cards {
            seq.serialize_element(card)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for CardCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CardCollectionVisitor;
        impl<'de> Visitor<'de> for CardCollectionVisitor {
            type Value = CardCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a card code")
            }
            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(CardCollection { cards: vec })
            }
        }

        deserializer.deserialize_seq(CardCollectionVisitor)
    }
}
