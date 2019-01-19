use super::cards::*;
use super::game::*;
use super::stringcode::StringCodes;

#[get("/game/<id>")]
pub fn get_game(id: String) -> String {
    serde_json::to_string_pretty(&Game::new()).unwrap()
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
