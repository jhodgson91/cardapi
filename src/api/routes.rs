use super::cards::*;
use super::game::*;
use super::models::HasModel;
use super::stringcode::StringCodes;

use super::common::*;

use rocket_contrib::json::JsonValue;

#[get("/game/new")]
pub fn new_game(conn: GamesDbConn) -> Option<JsonValue> {
    let game = Game::new();
    game.save(&conn).ok()?;
    Some(game.into())
}

#[get("/game/<id>")]
pub fn get_game(conn: GamesDbConn, id: String) -> Option<JsonValue> {
    let game = Game::load(&conn, id).ok()?;
    Some(JsonValue::from(serde_json::to_value(game).ok()?))
}

#[get("/game/<id>/deck", rank = 1)]
pub fn get_deck(conn: GamesDbConn, id: String) -> Option<JsonValue> {
    let game = Game::load(&conn, id).ok()?;
    Some(json!({
        "deck": game.deck.cards()
    }))
}

#[get("/game/<id>/<name>", rank = 2)]
pub fn get_pile(conn: GamesDbConn, id: String, name: String) -> Option<JsonValue> {
    let mut game = Game::load(&conn, id).ok()?;
    let pile = if game.has_pile(&name) {
        game.get_pile(&name)
    } else {
        game.new_pile(name.clone());
        game.save(&conn);
        game.get_pile(&name)
    }?;

    Some(json!({name: pile.cards()}))
}

#[put("/cards", data = "<selection>")]
pub fn cards(selection: CardSelection) -> Option<JsonValue> {
    let cards = CardCollection::from(selection);
    Some(JsonValue::from(serde_json::to_value(cards).ok()?))
}
