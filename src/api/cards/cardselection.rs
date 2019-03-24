use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CardSelection {
    Empty,
    All(bool),
    Top(usize),
    Bottom(usize),
    Random(usize),
    Filter {
        #[serde(default)]
        suits: StringCodes<CardSuit>,
        #[serde(default)]
        values: StringCodes<CardValue>,
    },
    Cards(CardCollection),
}

const LIMIT: u64 = 256;

use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, Status};
use rocket::{Data, Outcome, Outcome::*, Request};
use std::io::Read;

impl FromDataSimple for CardSelection {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let person_ct = ContentType::JSON;
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        match serde_json::from_str::<CardSelection>(string.as_str()) {
            Ok(result) => Outcome::Success(result),
            Err(e) => Failure((Status::InternalServerError, format!("{:?}", e))),
        }
    }
}
