#![feature(uniform_paths, proc_macro_hygiene, decl_macro)]
#![allow(warnings)]

#[macro_use]
extern crate rocket;

#[get("/cards?<suits>&<values>", rank = 1)]
fn cards_by_filter(suits: StringCodes<CardSuit>, values: StringCodes<CardValue>) -> String {
    let cards = CardSelection::from_all(CardSelection::Filter {suits, values});
    format!("Cards: {:?}", cards)
}

#[get("/cards?<suits>", rank = 2)]
fn cards_by_suit(suits: StringCodes<CardSuit>) -> String {
    let cards = CardSelection::from_all(CardSelection::Filter {suits, values: StringCodes::new()});
    format!("Cards: {:?}", cards)
}

#[get("/cards?<values>", rank = 3)]
fn cards_by_value(values: StringCodes<CardValue>) -> String{
    let cards = CardSelection::from_all(CardSelection::Filter {suits: StringCodes::new(), values});
    format!("Cards: {:?}", cards)
}

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

    use rocket::request::FromFormValue;
    use rocket::http::RawStr;

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

    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct StringCodes<T: HasStringCode> {
        _inner: Vec<T>,
    }

    impl<T: HasStringCode + Eq> StringCodes<T> {
        pub fn new() -> Self {
            StringCodes { _inner: Vec::new() }
        }

        pub fn from_str(s: String) -> Option<Self> {
            let codes: Vec<&str> = s.split(",").collect();
            let mut result: Vec<T> = Vec::new();
            for code in codes {
                result.push(T::from_str(code.to_string())?);
            }
            Some(StringCodes { _inner: result })
        }

        pub fn contains(&self, other: &T) -> bool {
            self._inner.contains(other)
        }

        pub fn len(&self) -> usize {
            self._inner.len()
        }
    }

    impl<'v, T: HasStringCode + Eq> FromFormValue<'v> for StringCodes<T> {
        type Error = super::CardAPIError;

        fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
            StringCodes::from_str(form_value.to_string()).ok_or(super::CardAPIError::NotFound)
        }
    }

    pub const CARD_CODES: &'static [&str] = &[
        "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS", "AD", "2D",
        "3D", "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD", "AC", "2C", "3C", "4C",
        "5C", "6C", "7C", "8C", "9C", "0C", "JC", "QC", "KC", "AH", "2H", "3H", "4H", "5H", "6H",
        "7H", "8H", "9H", "0H", "JH", "QH", "KH",
    ];
}

use diesel::prelude::*;

use cards::*;
use common::*;
use game::*;

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

    let s = StringCodes::<Card>::from_str("AH,2C".to_string()).unwrap();
    println!("{:?}", s);

    rocket::ignite().mount("/", routes![cards_by_filter, cards_by_suit, cards_by_value]).launch();


}
