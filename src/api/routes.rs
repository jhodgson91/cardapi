use super::cards::*;
use super::game::*;
use super::models::HasModel;
use super::stringcode::StringCodes;

use super::common::*;

use rocket_contrib::json::JsonValue;

#[get("/game/new")]
pub fn new_game(conn: GamesDbConn) -> String {
    let g = Game::new();
    g.save(&conn);
    serde_json::to_string_pretty(&g).unwrap()
}

#[get("/game/<id>")]
pub fn get_game(conn: GamesDbConn, id: String) -> Option<JsonValue> {
    let game = Game::load(&conn, id).ok()?;
    let a = serde_json::to_value(game).ok()?;
    Some(JsonValue::from(a))
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

#[get("/cards?<top>", rank = 1)]
pub fn cards_top(top: usize) -> String {
    let cards = CardSelection::from_all(CardSelection::Top(top));
    format!("Cards: {:?}", cards)
}

#[get("/cards?<bottom>", rank = 2)]
pub fn cards_bottom(bottom: usize) -> String {
    let cards = CardSelection::from_all(CardSelection::Bottom(bottom));
    format!("Cards: {:?}", cards)
}

#[get("/cards?<random>", rank = 3)]
pub fn cards_random(random: usize) -> String {
    let cards = CardSelection::from_all(CardSelection::Random(random));
    format!("Cards: {:?}", cards)
}

#[get("/cards?<suits>", rank = 4)]
pub fn cards_by_suit(suits: StringCodes<CardSuit>) -> String {
    let cards = CardSelection::from_all(CardSelection::Filter {
        suits,
        values: StringCodes::new(),
    });
    format!("Cards: {:?}", cards)
}

#[get("/cards?<values>", rank = 5)]
pub fn cards_by_value(values: StringCodes<CardValue>) -> String {
    let cards = CardSelection::from_all(CardSelection::Filter {
        suits: StringCodes::new(),
        values,
    });
    format!("Cards: {:?}", cards)
}

#[get("/cards?<suits>&<values>", rank = 6)]
pub fn cards_by_filter(suits: StringCodes<CardSuit>, values: StringCodes<CardValue>) -> String {
    let cards = CardSelection::from_all(CardSelection::Filter { suits, values });
    format!("Cards: {:?}", cards)
}
