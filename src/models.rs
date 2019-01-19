use super::schema::games;

use diesel::prelude::*;

pub trait HasModel {
    type Model;

    fn load(conn: &SqliteConnection, id: String) -> QueryResult<Self>
    where
        Self: std::marker::Sized;
    fn save(&self, conn: &SqliteConnection) -> QueryResult<usize>;

    fn from_model(m: Self::Model) -> Self;
    fn to_model(&self) -> Self::Model;
}

#[derive(Identifiable, Insertable, Queryable, AsChangeset, PartialEq, Debug)]
#[table_name = "games"]
pub struct Game {
    pub id: String,
    pub json: String,
}
