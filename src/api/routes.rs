use super::cards::*;
use super::game::*;
use super::models::HasModel;
use super::stringcode::StringCodes;

use super::common::*;
use super::routedata::DrawData;

use rocket_contrib::json::JsonValue;

#[get("/game/new")]
pub fn new_game(conn: GamesDbConn) -> Result<JsonValue, CardAPIError> {
    let game = Game::new();
    game.save(&conn)?;
    Ok(game.into())
}

#[get("/game/<id>")]
pub fn get_game(conn: GamesDbConn, id: String) -> Result<JsonValue, CardAPIError> {
    let game = Game::load(&conn, id)?;
    Ok(game.into())
}

#[get("/game/<id>/deck", rank = 1)]
pub fn get_deck(conn: GamesDbConn, id: String) -> Result<JsonValue, CardAPIError> {
    let game = Game::load(&conn, id)?;
    Ok(json!({
        "deck": game.deck()
    }))
}

#[get("/game/<id>/<name>", rank = 2)]
pub fn get_pile(conn: GamesDbConn, id: String, name: String) -> Result<JsonValue, CardAPIError> {
    let mut game = Game::load(&conn, id)?;
    let pile = if game.has_pile(&name) {
        game.get_pile(&name)
    } else {
        game.new_pile(name.clone());
        game.save(&conn)?;
        game.get_pile(&name)
    };

    Ok(json!({ name: pile }))
}

use rocket_contrib::json::Json;

#[put("/game/<id>/<name>/draw", data = "<drawdata>")]
pub fn draw_from_pile(
    conn: GamesDbConn,
    id: String,
    name: String,
    drawdata: Json<DrawData>,
) -> Result<JsonValue, CardAPIError> {
    let mut game = Game::load(&conn, id)?;
    let from = if name == "deck" {
        CollectionType::Deck
    } else {
        CollectionType::Pile(name)
    };

    let to = if drawdata.destination == "deck" {
        CollectionType::Deck
    } else {
        CollectionType::Pile(drawdata.destination.clone())
    };

    game.draw(from, to, &drawdata.selection)?;
    game.save(&conn);
    Ok(game.into())
}
