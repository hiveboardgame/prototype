use crate::db::schema::ratings;
use crate::db::util::DbPool;
use crate::model::ratings::ratings::*;
use crate::model::user::User;
use bb8::PooledConnection;
use diesel::{
    prelude::*, result::Error, AsChangeset, Associations, Identifiable, Insertable, QueryDsl,
    Queryable, Selectable,
};
use diesel_async::RunQueryDsl;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use hive_lib::{
    color::Color, game_result::GameResult,
};
use serde::{Deserialize, Serialize};
use skillratings::{
    glicko2::{glicko2, Glicko2Config, Glicko2Rating},
    Outcomes,
};

#[derive(Insertable, Debug)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    user_uid: String,
    played: i64,
    won: i64,
    lost: i64,
    draw: i64,
    rating: f64,
    deviation: f64,
    volatility: f64,
}

impl NewRating {
    pub fn for_uid(user: &str) -> Self {
        Self {
            user_uid: user.to_string(),
            played: 0,
            won: 0,
            lost: 0,
            draw: 0,
            rating: 1500.0,
            deviation: 350.0,
            volatility: 0.06,
        }
    }
}

#[derive(
    Associations,
    Identifiable,
    Queryable,
    Debug,
    Serialize,
    Deserialize,
    AsChangeset,
    Selectable,
    PartialEq,
)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(User, foreign_key = user_uid))]
#[diesel(table_name = ratings)]
#[diesel(primary_key(id))]
pub struct Rating {
    pub id: i32,
    pub user_uid: String,
    pub played: i64,
    pub won: i64,
    pub lost: i64,
    pub draw: i64,
    pub rating: f64,
    pub deviation: f64,
    pub volatility: f64,
}

impl Rating {
    pub async fn update(
        white_id: String,
        black_id: String,
        game_result: GameResult,
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        pool: &DbPool,
    ) -> Result<(), Error> {
        let white = User::find_by_uid(pool, &white_id).await?;
        let white_rating: Rating = Rating::belonging_to(&white).get_result(conn).await?;

        let black = User::find_by_uid(pool, &black_id).await?;
        let black_rating: Rating = Rating::belonging_to(&black).get_result(conn).await?;

        match game_result {
            GameResult::Draw => Rating::draw(&white_rating, &black_rating, conn).await?,
            GameResult::Winner(color) => {
                Rating::winner(color, &white_rating, &black_rating, conn).await?
            }
            GameResult::Unknown => unreachable!(
                "This function should not be called when there's no concrete game result"
            ),
        }
        Ok(())
    }

    fn calculate_glicko2(
        white_rating: &Rating,
        black_rating: &Rating,
        game_result: GameResult,
    ) -> (Glicko2Rating, Glicko2Rating) {
        let white_glicko = Glicko2Rating {
            rating: white_rating.rating,
            deviation: white_rating.deviation,
            volatility: white_rating.volatility,
        };

        let black_glicko = Glicko2Rating {
            rating: black_rating.rating,
            deviation: black_rating.deviation,
            volatility: black_rating.volatility,
        };

        match game_result {
            GameResult::Winner(winner) => {
                let outcome = {
                    if winner == Color::White {
                        Outcomes::WIN
                    } else {
                        Outcomes::LOSS
                    }
                };

                let config = Glicko2Config {
                    tau: 0.5,
                    ..Default::default()
                };

                return glicko2(&white_glicko, &black_glicko, &outcome, &config);
            }
            GameResult::Draw => {
                let outcome = Outcomes::DRAW;
                let config = Glicko2Config {
                    tau: 0.5,
                    ..Default::default()
                };

                return glicko2(&white_glicko, &black_glicko, &outcome, &config);
            }
            GameResult::Unknown => unreachable!(),
        }
    }

    async fn draw(
        white_rating: &Rating,
        black_rating: &Rating,
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> Result<(), Error> {
        let (white_glicko, black_glicko) =
            Rating::calculate_glicko2(white_rating, black_rating, GameResult::Draw);
        diesel::update(ratings::table.find(black_rating.id))
            .set((
                played.eq(played + 1),
                draw.eq(draw + 1),
                rating.eq(black_glicko.rating),
                deviation.eq(black_glicko.deviation),
                volatility.eq(black_glicko.volatility),
            ))
            .execute(conn)
            .await?;

        diesel::update(ratings::table.find(white_rating.id))
            .set((
                played.eq(played + 1),
                draw.eq(draw + 1),
                rating.eq(white_glicko.rating),
                deviation.eq(white_glicko.deviation),
                volatility.eq(white_glicko.volatility),
            ))
            .execute(conn)
            .await?;
        Ok(())
    }

    async fn winner(
        winner: Color,
        white_rating: &Rating,
        black_rating: &Rating,
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> Result<(), Error> {
        let (white_glicko, black_glicko) =
            Rating::calculate_glicko2(white_rating, black_rating, GameResult::Winner(winner));

        let (white_won, white_lost) = {
            if winner == Color::White {
                (1, 0)
            } else {
                (0, 1)
            }
        };

        diesel::update(ratings::table.find(black_rating.id))
            .set((
                played.eq(played + 1),
                won.eq(won + white_won),
                lost.eq(lost + white_lost),
                rating.eq(black_glicko.rating),
                deviation.eq(black_glicko.deviation),
                volatility.eq(black_glicko.volatility),
            ))
            .execute(conn)
            .await?;

        diesel::update(ratings::table.find(white_rating.id))
            .set((
                played.eq(played + 1),
                won.eq(won + white_lost),
                lost.eq(lost + white_won),
                rating.eq(white_glicko.rating),
                deviation.eq(white_glicko.deviation),
                volatility.eq(white_glicko.volatility),
            ))
            .execute(conn)
            .await?;
        Ok(())
    }
}
