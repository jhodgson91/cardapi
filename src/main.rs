#![feature(uniform_paths)]
#![allow(warnings)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2;
use diesel::sqlite::SqliteConnection;

mod card;
mod cardcollection;
mod cardselection;
mod cardsuit;
mod cardvalue;
mod models;
mod schema;
mod common {
    pub trait HasStringCode {
        fn from_str(s: String) -> Option<Self>
        where
            Self: std::marker::Sized;
        fn to_str(&self) -> String;
    }


    pub enum CardAPIError {
        DieselError(diesel::result::Error),
        NotFound(String),
        AlreadyExists(String)
    }

    impl From<diesel::result::Error> for CardAPIError {
        fn from(e: diesel::result::Error) -> CardAPIError {
            CardAPIError::DieselError(e)
        }
    }

    pub const CARD_CODES: &'static [&str] = &[
        "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS", "AD", "2D",
        "3D", "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD", "AC", "2C", "3C", "4C",
        "5C", "6C", "7C", "8C", "9C", "0C", "JC", "QC", "KC", "AH", "2H", "3H", "4H", "5H", "6H",
        "7H", "8H", "9H", "0H", "JH", "QH", "KH",
    ];
}

use card::Card;
use diesel::prelude::*;
use std::convert::AsMut;

#[derive(Serialize, Deserialize)]
struct Pile {
    pub _name: String,
    _deck_id: String,
    _cards: Vec<Card>,
}

impl CardCollection for Pile {
    fn id(&self) -> String {
        format!("{}::{}", self._deck_id, self._name)
    }

    fn cards(&self) -> &Vec<Card> {
        &self._cards
    }

    fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self._cards
    }

    fn load(db: &SqliteConnection, identifier: &str) -> QueryResult<Self> {
        use schema::piles::dsl::*;

        let data = piles.find(identifier).get_result::<models::Pile>(db)?;
        Ok(Pile {
            _name: data.name,
            _deck_id: data.deck_id,
            _cards: serde_json::from_str(data.cards.as_str()).unwrap(),
        })
    }

    fn save(&self, db: &SqliteConnection) -> QueryResult<usize> {
        use schema::piles::dsl::*;

        let data = models::Pile {
            id: self.id(),
            name: self._name.clone(),
            deck_id: self._deck_id.clone(),
            cards: serde_json::to_string(self.cards()).expect("Pile Save - Serialization Error"),
        };

        let mut result = diesel::update(piles.find(self.id()))
            .set(&data)
            .execute(db)?;
        if result == 0 {
            result = diesel::insert_into(piles).values(&data).execute(db)?;
        }
        Ok(result)
    }
}

use cardcollection::*;
use cardselection::*;

#[derive(Serialize, Deserialize)]
struct Deck {
    _id: String,
    _cards: Vec<Card>,
    _piles: Vec<Pile>,
}

impl Deck {
    pub fn new(conn: &SqliteConnection, selection: CardSelection) -> QueryResult<Deck> {
        let result = Deck {
            _id: Deck::new_id(),
            _cards: CardSelection::from_all(selection),
            _piles: Vec::new(),
        };

        result.save(conn)?;
        Ok(result)
    }
    /*
    pub fn new_pile(
        &mut self,
        db: &SqliteConnection,
        name: String
    ) -> Result<&Pile, common::CardAPIError> {
        let result = Pile {
            _name: name,
            _deck_id: self.id(),
            _cards: Vec::new(),
        };

        result.save(db)?;
        self._piles.push(result);
        Ok(&self._piles[self._piles.len() - 1])
    }
    */

    pub fn get_pile(&self, name: String) -> Option<&Pile> {
        self._piles.iter().find(|p| p._name == name)
    }

    fn new_id() -> String {
        use rand::seq::IteratorRandom;
        use rand::thread_rng;

        (0..26)
            .chain(32..58)
            .map(|x| (x + 'A' as u8) as char)
            .choose_multiple(&mut thread_rng(), 12)
            .iter()
            .collect()
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap().as_str())
    }
}

impl CardCollection for Deck {
    fn id(&self) -> String {
        self._id.clone()
    }

    fn cards(&self) -> &Vec<Card> {
        &self._cards
    }

    fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self._cards
    }

    fn load(db: &SqliteConnection, identifier: &str) -> QueryResult<Self> {
        use schema::decks::dsl::*;

        let data = decks.find(identifier).get_result::<models::Deck>(db)?;
        let children = models::Pile::belonging_to(&data).load::<models::Pile>(db)?;

        Ok(Deck {
            _id: data.id,
            _cards: serde_json::from_str(data.cards.as_str()).unwrap(),
            _piles: children
                .iter()
                .map(|p| Pile {
                    _name: p.name.clone(),
                    _deck_id: p.deck_id.clone(),
                    _cards: serde_json::from_str(p.cards.as_str()).unwrap(),
                })
                .collect(),
        })
    }

    fn save(&self, db: &SqliteConnection) -> QueryResult<usize> {
        use schema::decks::dsl::*;

        let data = models::Deck {
            id: self._id.clone(),
            cards: serde_json::to_string(&self._cards).expect("Deck Save - Serialization error"),
        };

        let mut result = diesel::update(decks.find(self.id()))
            .set(&data)
            .execute(db)?;
        if result == 0 {
            result = diesel::insert_into(decks).values(&data).execute(db)?;
        }

        let mut pileresult = 0;
        for p in &self._piles {
            pileresult += p.save(db)?;
        }

        Ok(result + pileresult)
    }
}

type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("DB Pool")
}

fn main() {
    let pool = init_pool();
    let conn = pool.get().unwrap();

    let mut deck = Deck::load(&conn, "OBPoEbcQjtvr").expect("Failed to make the deck!");
    //let pile = deck.new_pile(&conn, "discard".to_string(), CardSelection::Random(5));
    if let Some(pile) = deck.get_pile("discard".to_string()) {
        println!("{}", pile._name);
    }

    println!("{}", deck);
}
