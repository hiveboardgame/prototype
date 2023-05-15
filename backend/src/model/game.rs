use std::str::FromStr;

use crate::db::schema::games;
use crate::db::schema::games::dsl::*;
use crate::db::util::{get_conn, DbPool};
use crate::model::games_users::GameUser;
use diesel::{prelude::*, result::Error, Identifiable, Insertable, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use hive_lib::{
    color::Color, game_control::GameControl, game_result::GameResult, game_status::GameStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug)]
#[diesel(table_name = games)]
// TODO: use our types
pub struct NewGame {
    pub black_uid: String, // uid of user
    pub game_status: String,
    pub game_type: String,
    pub history: String,
    pub game_control_history: String,
    pub ranked: bool,
    pub tournament_queen_rule: bool,
    pub turn: i32,
    pub white_uid: String, // uid of user
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, AsChangeset, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = games)]
pub struct Game {
    pub id: i32,
    pub black_uid: String, // uid of user
    pub game_status: String,
    pub game_type: String,
    pub history: String, //"piece pos;piece pos;piece pos;"
    pub game_control_history: String,
    pub ranked: bool,
    pub tournament_queen_rule: bool,
    pub turn: i32,
    pub white_uid: String, // uid of user
}

impl Game {
    pub async fn create(new_game: &NewGame, pool: &DbPool) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        let game: Game = new_game.insert_into(games::table).get_result(conn).await?;
        let game_user_white = GameUser::new(game.id, game.white_uid.clone());
        game_user_white.insert(pool).await?;
        let game_user_black = GameUser::new(game.id, game.black_uid.clone());
        game_user_black.insert(pool).await?;
        Ok(game)
    }

    pub async fn make_move(
        &self,
        mut board_move: String,
        new_game_status: String,
        pool: &DbPool,
    ) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        if board_move.chars().last().unwrap_or(' ') != ';' {
            board_move = format!("{board_move};");
        }
        let mut game_control_string = String::new();
        if self.has_unanswered_game_control() {
            let gc = match self.last_game_control() {
                Some(GameControl::TakebackRequest(color)) => {
                    GameControl::TakebackReject(Color::from(color.opposite()))
                }
                Some(GameControl::DrawOffer(color)) => {
                    GameControl::DrawReject(Color::from(color.opposite()))
                }
                _ => unreachable!(),
            };
            game_control_string = format!("{}. {gc};", self.turn);
        }
        diesel::update(games::table.find(self.id))
            .set((
                history.eq(history.concat(board_move)),
                turn.eq(turn + 1),
                game_status.eq(new_game_status),
                game_control_history.eq(game_control_history.concat(game_control_string)),
            ))
            .get_result(conn)
            .await
    }

    pub fn has_unanswered_game_control(&self) -> bool {
        if let Some(gc) = self.last_game_control() {
            return matches!(
                gc,
                GameControl::TakebackRequest(_) | GameControl::DrawOffer(_)
            );
        }
        false
    }

    pub fn last_game_control(&self) -> Option<GameControl> {
        if let Some(last) = self.game_control_history.split_terminator(';').last() {
            if let Some(gc) = last.split(' ').last() {
                return Some(
                    GameControl::from_str(gc)
                        .expect("Could not get GameControl from game_control_history"),
                );
            }
        }
        None
    }

    pub async fn write_game_control(
        &self,
        game_control: GameControl,
        pool: &DbPool,
    ) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        let game_control_string = format!("{}. {game_control};", self.turn);
        diesel::update(games::table.find(self.id))
            .set(game_control_history.eq(game_control_history.concat(game_control_string)))
            .get_result(conn)
            .await
    }

    pub async fn accept_takeback(
        &self,
        new_history: String,
        new_game_status: String,
        game_control: GameControl,
        pool: &DbPool,
    ) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        let game_control_string = format!("{}. {game_control};", self.turn);

        diesel::update(games::table.find(self.id))
            .set((
                history.eq(new_history),
                turn.eq(turn - 1),
                game_status.eq(new_game_status),
                game_control_history.eq(game_control_history.concat(game_control_string)),
            ))
            .get_result(conn)
            .await
    }

    pub async fn resign(
        &self,
        game_control: GameControl,
        new_game_status: GameStatus,
        winner: Color,
        pool: &DbPool,
    ) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        let game_control_string = format!("{}. {game_control};", self.turn);

        diesel::update(games::table.find(self.id))
            .set((
                game_status.eq(new_game_status.to_string()),
                game_control_history.eq(game_control_history.concat(game_control_string)),
            ))
            .get_result(conn)
            .await
    }

    pub async fn accept_draw(
        &self,
        game_control: GameControl,
        pool: &DbPool,
    ) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        let game_control_string = format!("{}. {game_control};", self.turn);
        diesel::update(games::table.find(self.id))
            .set((
                game_control_history.eq(game_control_history.concat(game_control_string)),
                game_status.eq(GameStatus::Finished(GameResult::Draw).to_string()),
            ))
            .get_result(conn)
            .await
    }

    pub async fn set_status(&self, status: GameStatus, pool: &DbPool) -> Result<Game, Error> {
        let conn = &mut get_conn(pool).await?;
        diesel::update(games::table.find(self.id))
            .set(game_status.eq(status.to_string()))
            .get_result(conn)
            .await
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
