#[derive(Debug)]
pub enum CardAPIError {
    DieselError(diesel::result::Error),
    NotFound(String),
    AlreadyExists,
    NotEnoughCards,
    CardNotInCollection,
    CardAlreadyInCollection,
}

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::io::Cursor;
impl Responder<'static> for CardAPIError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        match self {
            CardAPIError::NotFound(s) => Response::build()
                .status(Status::raw(404))
                .sized_body(Cursor::new(format!("Not found: {}", s)))
                .ok(),
            CardAPIError::NotEnoughCards => Response::build()
                .status(Status::raw(500))
                .sized_body(Cursor::new("Not enough cards"))
                .ok(),
            CardAPIError::DieselError(e) => Response::build()
                .status(Status::raw(500))
                .sized_body(Cursor::new(format!("Database Error: {}", e)))
                .ok(),
            _ => Ok(Response::new()),
        }
    }
}

impl From<diesel::result::Error> for CardAPIError {
    fn from(e: diesel::result::Error) -> CardAPIError {
        CardAPIError::DieselError(e)
    }
}
