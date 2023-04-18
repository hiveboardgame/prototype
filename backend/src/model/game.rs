use crate::db::schema::games;
use crate::db::schema::games::dsl::games as games_table;
use crate::db::util::{get_conn, DbPool};
use crate::server_error::ServerError;
use diesel::{result::Error, Identifiable, Insertable, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use hive_lib::game_status::GameStatus;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct NewGame<'a> {
    pub game_type: &'a str,
    pub game_status: &'a str,
    pub black: &'a str,
    pub white: &'a str,
    pub tournament_rules: bool,
}

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(id))]
pub struct Game {
    pub id: i32,
    pub game_type: String,
    pub game_status: String,
    pub turn: i32,
    pub history: Vec<String>,
    pub white: String, // uid of user
    pub black: String, // uid of user
    pub tournament_rules: bool,
}

impl Game {
    pub fn new() -> Result<Game, ServerError> {
        Ok(Self {
            id: 0,
            game_type: "Base+MLP".to_string(),
            game_status: "NotStarted".to_string(),
            turn: 0,
            history: Vec::new(),
            black: String::new(),
            white: String::new(),
            tournament_rules: true,
        })
    }
}
