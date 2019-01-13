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

mod cards;
mod game;
mod models;
mod schema;
mod common {
    #[derive(Debug)]
    pub enum CardAPIError {
        DieselError(diesel::result::Error),
        NotFound,
        AlreadyExists,
    }

    impl From<diesel::result::Error> for CardAPIError {
        fn from(e: diesel::result::Error) -> CardAPIError {
            match e {
                diesel::result::Error::NotFound => CardAPIError::NotFound,
                _ => CardAPIError::DieselError(e),
            }
        }
    }

    pub trait HasStringCode {
    fn from_str(s: String) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn to_str(&self) -> String;
    }

    pub const CARD_CODES: &'static [&str] = &[
    "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS", "AD", "2D",
    "3D", "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD", "AC", "2C", "3C", "4C",
    "5C", "6C", "7C", "8C", "9C", "0C", "JC", "QC", "KC", "AH", "2H", "3H", "4H", "5H", "6H",
    "7H", "8H", "9H", "0H", "JH", "QH", "KH",
    ];
}

use diesel::prelude::*;

use game::*;
use cards::*;
use common::*;

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

    let mut game = Game::new();
    game.new_pile("discard".to_string());
    println!("{:?}", game);
    game.move_cards(CollectionType::Deck, CollectionType::Pile("discard".to_string()), CardSelection::Random(10));

    println!("{:?}", game);
}
