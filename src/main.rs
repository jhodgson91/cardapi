#![feature(proc_macro_hygiene, decl_macro)]
#![allow(warnings)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2;
use diesel::sqlite::SqliteConnection;

mod api;
mod cards;
mod game;
mod models;
mod schema;
mod stringcode;
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

    pub const CARD_CODES: &'static [&str] = &[
        "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS", "AD", "2D",
        "3D", "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD", "AC", "2C", "3C", "4C",
        "5C", "6C", "7C", "8C", "9C", "0C", "JC", "QC", "KC", "AH", "2H", "3H", "4H", "5H", "6H",
        "7H", "8H", "9H", "0H", "JH", "QH", "KH",
    ];

    use rocket_contrib::databases::diesel;

    #[database("sqlite_games")]
    pub struct GamesDbConn(diesel::SqliteConnection);

}

use diesel::prelude::*;

use cards::*;
use common::*;
use game::*;
use models::HasModel;
use stringcode::*;

type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("DB Pool")
}

fn main() {
    rocket::ignite()
        .attach(common::GamesDbConn::fairing())
        .mount(
            "/",
            routes![
                api::routes::cards_by_filter,
                api::routes::cards_by_suit,
                api::routes::cards_by_value,
                api::routes::cards_random,
                api::routes::cards_top,
                api::routes::cards_bottom,
                api::routes::get_game,
                api::routes::new_game,
            ],
        )
        .launch();
}
