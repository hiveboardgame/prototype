use crate::db::schema::games;
use crate::db::schema::games::dsl::games as games_table;
use crate::db::schema::games::dsl::*;
use crate::db::util::{get_conn, DbPool};
use crate::server_error::ServerError;
use diesel::{prelude::*, result::Error, Identifiable, Insertable, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug)]
#[diesel(table_name = games)]
// TODO use our types
pub struct NewGame {
    pub black_uid: String, // uid of user
    pub game_status: String,
    pub game_type: String,
    pub history: String,
    pub tournament_queen_rule: bool,
    pub turn: i32,
    pub white_uid: String, // uid of user
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = games)]
pub struct Game {
    pub id: i32,
    pub black_uid: String, // uid of user
    pub game_status: String,
    pub game_type: String,
    pub history: String,
    pub tournament_queen_rule: bool,
    pub turn: i32,
    pub white_uid: String, // uid of user
}

//"move;move;move;"

impl Game {
    pub async fn create(new_game: &NewGame, pool: &DbPool) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        new_game.insert_into(games::table).get_result(conn).await
    }

    pub async fn make_move(&self, board_move: String, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        // append to history
        let game = diesel::update(games::table.find(self.id))
            .set(history.eq("wasd"))
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn get(other_id: i32, pool: &DbPool) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        games::table.find(other_id).first(conn).await
    }

    pub async fn delete(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        diesel::delete(games::table.find(self.id))
            .execute(conn)
            .await?;
        Ok(())
    }
}
