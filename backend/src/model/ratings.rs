use crate::db::schema::ratings;
use crate::db::util::get_conn;
use crate::model::ratings::dsl::ratings as ratings_table;
use crate::model::ratings::ratings::*;
use crate::model::user::User;
use crate::DbPool;
use bb8::PooledConnection;
use diesel::{
    prelude::*, result::Error, AsChangeset, Associations, Identifiable, Insertable, QueryDsl,
    Queryable, Selectable,
};
use diesel_async::RunQueryDsl;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use hive_lib::{color::Color, game_result::GameResult};
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
    pub async fn for_uid(uid: &str, pool: &DbPool) -> Result<Self, Error> {
        let conn = &mut get_conn(pool).await?;
        ratings_table.filter(user_uid.eq(uid)).first(conn).await
    }

    pub async fn update(
        rated: bool,
        white_id: String,
        black_id: String,
        game_result: GameResult,
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> Result<Option<(f64, f64)>, Error> {
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white_id))
            .first(conn)
            .await?;
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black_id))
            .first(conn)
            .await?;

        match game_result {
            GameResult::Draw => Rating::draw(rated, &white_rating, &black_rating, conn).await,
            GameResult::Winner(color) => {
                Rating::winner(rated, color, &white_rating, &black_rating, conn).await
            }
            GameResult::Unknown => unreachable!(
                "This function should not be called when there's no concrete game result"
            ),
        }
    }

    fn calculate_glicko2(
        white_rating: &Rating,
        black_rating: &Rating,
        game_result: GameResult,
    ) -> (Glicko2Rating, Glicko2Rating, f64, f64) {
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

        let config = Glicko2Config {
            tau: 0.5,
            ..Default::default()
        };
        let outcome = match game_result {
            GameResult::Winner(winner) => {
                if winner == Color::White {
                    Outcomes::WIN
                } else {
                    Outcomes::LOSS
                }
            }
            GameResult::Draw => Outcomes::DRAW,
            GameResult::Unknown => unreachable!(),
        };
        let (white_glicko_new, black_glicko_new) =
            glicko2(&white_glicko, &black_glicko, &outcome, &config);
        (
            white_glicko_new,
            black_glicko_new,
            white_glicko_new.rating - white_glicko.rating,
            black_glicko_new.rating - black_glicko.rating,
        )
    }

    async fn draw(
        rated: bool,
        white_rating: &Rating,
        black_rating: &Rating,
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> Result<Option<(f64, f64)>, Error> {
        if rated {
            let (white_glicko, black_glicko, white_rating_change, black_rating_change) =
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
            Ok(Some((white_rating_change, black_rating_change)))
        } else {
            diesel::update(ratings::table.find(black_rating.id))
                .set((played.eq(played + 1), draw.eq(draw + 1)))
                .execute(conn)
                .await?;

            diesel::update(ratings::table.find(white_rating.id))
                .set((played.eq(played + 1), draw.eq(draw + 1)))
                .execute(conn)
                .await?;
            Ok(None)
        }
    }

    async fn winner(
        rated: bool,
        winner: Color,
        white_rating: &Rating,
        black_rating: &Rating,
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> Result<Option<(f64, f64)>, Error> {
        let (white_won, white_lost) = {
            if winner == Color::White {
                (1, 0)
            } else {
                (0, 1)
            }
        };

        if rated {
            let (white_glicko, black_glicko, white_rating_change, black_rating_change) =
                Rating::calculate_glicko2(white_rating, black_rating, GameResult::Winner(winner));

            diesel::update(ratings::table.find(white_rating.id))
                .set((
                    played.eq(played + 1),
                    won.eq(won + white_won),
                    lost.eq(lost + white_lost),
                    rating.eq(white_glicko.rating),
                    deviation.eq(white_glicko.deviation),
                    volatility.eq(white_glicko.volatility),
                ))
                .execute(conn)
                .await?;

            diesel::update(ratings::table.find(black_rating.id))
                .set((
                    played.eq(played + 1),
                    won.eq(won + white_lost),
                    lost.eq(lost + white_won),
                    rating.eq(black_glicko.rating),
                    deviation.eq(black_glicko.deviation),
                    volatility.eq(black_glicko.volatility),
                ))
                .execute(conn)
                .await?;
            Ok(Some((white_rating_change, black_rating_change)))
        } else {
            diesel::update(ratings::table.find(white_rating.id))
                .set((
                    played.eq(played + 1),
                    won.eq(won + white_won),
                    lost.eq(lost + white_lost),
                ))
                .execute(conn)
                .await?;

            diesel::update(ratings::table.find(black_rating.id))
                .set((
                    played.eq(played + 1),
                    won.eq(won + white_lost),
                    lost.eq(lost + white_won),
                ))
                .execute(conn)
                .await?;
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::db::util::get_conn;
    use crate::model::ratings::dsl::ratings as ratings_table;
    use crate::model::ratings::user_uid;
    use crate::model::ratings::Rating;
    use crate::{
        accept_challenge, game_control, make_challenge, make_rated_challenge, make_user, play_turn,
    };
    use crate::{api::game::game_state_response::GameStateResponse, test::DBTest};
    use actix_web::test::{self, TestRequest};
    use diesel::{prelude::*, QueryDsl};
    use diesel_async::RunQueryDsl;
    use hive_lib::color::Color;
    use hive_lib::game_control::GameControl;
    use hive_lib::game_result::GameResult;
    use hive_lib::game_status::GameStatus;
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn resign_rated_game(ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_rated_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(game.game_id, white.uid.clone(), "Resign", "White", &app);
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::Finished(hive_lib::game_result::GameResult::Winner(
                hive_lib::color::Color::Black
            ))
        );
        assert!(game.black_rating_change.is_some());
        assert!(game.black_rating_change.unwrap() > 0.0);
        assert!(game.white_rating_change.is_some());
        assert!(game.white_rating_change.unwrap() < 0.0);
        let mut conn = get_conn(&ctx.pool).await.unwrap();
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white.uid))
            .first(&mut conn)
            .await
            .unwrap();
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black.uid))
            .first(&mut conn)
            .await
            .unwrap();
        assert!(white_rating.rating < black_rating.rating);
        assert_eq!(white_rating.lost, 1);
        assert_eq!(black_rating.won, 1);
        assert_eq!(white_rating.played, 1);
        assert_eq!(black_rating.played, 1);
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn resign_unrated_game(ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(game.game_id, white.uid.clone(), "Resign", "White", &app);
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::Finished(hive_lib::game_result::GameResult::Winner(
                hive_lib::color::Color::Black
            ))
        );
        let mut conn = get_conn(&ctx.pool).await.unwrap();
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white.uid))
            .first(&mut conn)
            .await
            .unwrap();
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black.uid))
            .first(&mut conn)
            .await
            .unwrap();
        assert!(game.black_rating_change.is_none());
        assert!(game.white_rating_change.is_none());
        assert_eq!(white_rating.rating, black_rating.rating);
        assert_eq!(white_rating.lost, 1);
        assert_eq!(black_rating.won, 1);
        assert_eq!(white_rating.played, 1);
        assert_eq!(black_rating.played, 1);
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn draw_rated_game(ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_rated_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(game.game_id, white.uid.clone(), "DrawOffer", "White", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawOffer(Color::White))
        );
        let game = game_control!(game.game_id, black.uid.clone(), "DrawAccept", "Black", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawAccept(Color::Black))
        );
        assert_eq!(game.game_status, GameStatus::Finished(GameResult::Draw));
        let mut conn = get_conn(&ctx.pool).await.unwrap();
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white.uid))
            .first(&mut conn)
            .await
            .unwrap();
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black.uid))
            .first(&mut conn)
            .await
            .unwrap();
        assert!(game.black_rating_change.is_some());
        assert_eq!(game.black_rating_change.unwrap(), 0.0);
        assert!(game.white_rating_change.is_some());
        assert_eq!(game.white_rating_change.unwrap(), 0.0);
        assert_eq!(white_rating.rating, black_rating.rating);
        assert_ne!(white_rating.deviation, 350.0);
        assert_ne!(black_rating.deviation, 350.0);
        assert_eq!(white_rating.won, 0);
        assert_eq!(white_rating.lost, 0);
        assert_eq!(black_rating.won, 0);
        assert_eq!(black_rating.lost, 0);
        assert_eq!(white_rating.draw, 1);
        assert_eq!(black_rating.draw, 1);
        assert_eq!(white_rating.played, 1);
        assert_eq!(black_rating.played, 1);
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn draw_unrated_game(ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(game.game_id, white.uid.clone(), "DrawOffer", "White", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawOffer(Color::White))
        );
        let game = game_control!(game.game_id, black.uid.clone(), "DrawAccept", "Black", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawAccept(Color::Black))
        );
        assert_eq!(game.game_status, GameStatus::Finished(GameResult::Draw));
        let mut conn = get_conn(&ctx.pool).await.unwrap();
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white.uid))
            .first(&mut conn)
            .await
            .unwrap();
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black.uid))
            .first(&mut conn)
            .await
            .unwrap();
        assert!(game.black_rating_change.is_none());
        assert!(game.white_rating_change.is_none());
        assert_eq!(white_rating.rating, black_rating.rating);
        assert_eq!(white_rating.deviation, 350.0);
        assert_eq!(black_rating.deviation, 350.0);
        assert_eq!(white_rating.won, 0);
        assert_eq!(white_rating.lost, 0);
        assert_eq!(black_rating.won, 0);
        assert_eq!(black_rating.lost, 0);
        assert_eq!(white_rating.draw, 1);
        assert_eq!(black_rating.draw, 1);
        assert_eq!(white_rating.played, 1);
        assert_eq!(black_rating.played, 1);
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn play_rated_game(ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_rated_challenge!(white.uid.clone(), "White", &app);
        let mut game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let moves = vec![
            ["wL", "."],
            ["bL", "/wL"],
            ["wM", "wL/"],
            ["bQ", "bL\\"],
            ["wQ", "\\wL"],
            ["bP", "bQ\\"],
            ["wA1", "wM\\"],
            ["bA1", "-bL"],
            ["wM", "/bQ"],
            ["bA1", "\\wQ"],
            ["wB1", "/wM"],
            ["bB1", "bQ-"],
            ["wA1", "bB1-"],
            ["bB2", "bP\\"],
            ["wB1", "wM"],
            ["bA1", "wL\\"],
            ["wB1", "-bQ"],
        ];
        for (i, mov) in moves.iter().enumerate() {
            if i % 2 == 0 {
                game = play_turn!(game.game_id, white.uid.clone(), mov, &app);
            } else {
                game = play_turn!(game.game_id, black.uid.clone(), mov, &app);
            }
        }
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::Finished(hive_lib::game_result::GameResult::Winner(
                hive_lib::color::Color::White
            ))
        );
        assert!(game.black_rating_change.is_some());
        assert!(game.black_rating_change.unwrap() < 0.0);
        assert!(game.white_rating_change.is_some());
        assert!(game.white_rating_change.unwrap() > 0.0);
        let mut conn = get_conn(&ctx.pool).await.unwrap();
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white.uid))
            .first(&mut conn)
            .await
            .unwrap();
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black.uid))
            .first(&mut conn)
            .await
            .unwrap();
        assert!(white_rating.rating > black_rating.rating);
        assert_eq!(white_rating.won, 1);
        assert_eq!(white_rating.lost, 0);
        assert_eq!(black_rating.won, 0);
        assert_eq!(black_rating.lost, 1);
        assert_eq!(white_rating.draw, 0);
        assert_eq!(black_rating.draw, 0);
        assert_eq!(white_rating.played, 1);
        assert_eq!(black_rating.played, 1);
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn play_unrated_game(ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let mut game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let moves = vec![
            ["wL", "."],
            ["bL", "/wL"],
            ["wM", "wL/"],
            ["bQ", "bL\\"],
            ["wQ", "\\wL"],
            ["bP", "bQ\\"],
            ["wA1", "wM\\"],
            ["bA1", "-bL"],
            ["wM", "/bQ"],
            ["bA1", "\\wQ"],
            ["wB1", "/wM"],
            ["bB1", "bQ-"],
            ["wA1", "bB1-"],
            ["bB2", "bP\\"],
            ["wB1", "wM"],
            ["bA1", "wL\\"],
            ["wB1", "-bQ"],
        ];
        for (i, mov) in moves.iter().enumerate() {
            if i % 2 == 0 {
                game = play_turn!(game.game_id, white.uid.clone(), mov, &app);
            } else {
                game = play_turn!(game.game_id, black.uid.clone(), mov, &app);
            }
        }
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::Finished(hive_lib::game_result::GameResult::Winner(
                hive_lib::color::Color::White
            ))
        );
        let mut conn = get_conn(&ctx.pool).await.unwrap();
        let white_rating: Rating = ratings_table
            .filter(user_uid.eq(white.uid))
            .first(&mut conn)
            .await
            .unwrap();
        let black_rating: Rating = ratings_table
            .filter(user_uid.eq(black.uid))
            .first(&mut conn)
            .await
            .unwrap();
        assert!(game.black_rating_change.is_none());
        assert!(game.white_rating_change.is_none());
        assert_eq!(white_rating.rating, black_rating.rating);
        assert_eq!(white_rating.won, 1);
        assert_eq!(white_rating.lost, 0);
        assert_eq!(black_rating.won, 0);
        assert_eq!(black_rating.lost, 1);
        assert_eq!(white_rating.draw, 0);
        assert_eq!(black_rating.draw, 0);
        assert_eq!(white_rating.played, 1);
        assert_eq!(black_rating.played, 1);
    }
}
