use actix_web::{
    get, http, post,
    web::{self, Json},
    HttpResponse,
};
use names::{Generator, Name};
use serde::Deserialize;

use crate::challenge::game_challenge_response::GameChallengeResponse;
use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;

#[get("/user/{uid}/games")]
pub async fn get_user_games(
    uid: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::find_by_uid(pool.get_ref(), uid.as_ref()).await?;
    let games = user.get_games(&pool).await?;
    Ok(HttpResponse::Ok().json(games))
}

#[cfg(test)]
mod tests {
    use crate::api::game::game_state_response::GameStateResponse;
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::{accept_challenge, make_challenge, make_guest_user, make_user, test::DBTest};

    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn user_games(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let resp = TestRequest::get()
            .uri("/api/user/black/games")
            .send_request(&app)
            .await;
        //Get a user's games that has no games started
        assert!(resp.status().is_success(), "getting games failed");
        //Make a game between white and black and check that they both have the same game
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let resp_white = TestRequest::get()
            .uri("/api/user/white/games")
            .send_request(&app)
            .await;
        let resp_black = TestRequest::get()
            .uri("/api/user/black/games")
            .send_request(&app)
            .await;
        let body_white = test::read_body(resp_white).await;
        let body_black = test::read_body(resp_black).await;
        assert_eq!(body_white, body_black)
    }
}
