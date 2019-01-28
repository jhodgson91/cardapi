#![feature(proc_macro_hygiene, decl_macro)]
#![allow(warnings)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

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
        NotEnoughCards,
        CardNotInCollection,
        CardAlreadyInCollection,
    }

    impl From<diesel::result::Error> for CardAPIError {
        fn from(e: diesel::result::Error) -> CardAPIError {
            match e {
                diesel::result::Error::NotFound => CardAPIError::NotFound,
                _ => CardAPIError::DieselError(e),
            }
        }
    }

    use rocket_contrib::databases::diesel;

    #[database("sqlite_games")]
    pub struct GamesDbConn(diesel::SqliteConnection);

}

use cards::{CardCollection, CardSelection};

fn main() {
    rocket::ignite()
        .attach(common::GamesDbConn::fairing())
        .mount(
            "/",
            routes![
                api::routes::get_game,
                api::routes::get_pile,
                api::routes::get_deck,
                api::routes::new_game,
                api::routes::draw_from_pile,
            ],
        )
        .launch();
}
