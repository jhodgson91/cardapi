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
mod schema;

use api::*;

#[database("sqlite_games")]
pub struct GamesDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(GamesDbConn::fairing())
        .mount(
            "/",
            routes![
                api::get_game,
                api::get_pile,
                api::new_game,
                api::draw_from_pile,
            ],
        )
        .launch();
}
