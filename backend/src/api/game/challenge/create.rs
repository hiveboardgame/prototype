use actix_web::{
    post,
    web::{self, Json},
};

use crate::api::game::challenge::game_challenge_response::GameChallengeResponse;
use crate::challenge::game_challenge_response::NewGameChallengeRequest;
use crate::{
    db::util::DbPool, extractors::auth::AuthenticatedUser, model::challenge::GameChallenge,
    server_error::ServerError,
};

#[post("/game/challenge")]
pub async fn create_game_challenge(
    game: web::Json<NewGameChallengeRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<Json<GameChallengeResponse>, ServerError> {
    let challenge = GameChallenge::create(&auth_user, &game, &pool).await?;
    let response = GameChallengeResponse::from_model(&challenge, &pool).await?;
    Ok(web::Json(response))
}

#[cfg(test)]
mod tests {
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::{accept_challenge, make_challenge, make_user};
    use crate::{api::game::game_state_response::GameStateResponse, test::DBTest};
    use actix_web::test::{self, TestRequest};
    use hive_lib::game_status::GameStatus;
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn create(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        assert_eq!(game.game_status, GameStatus::NotStarted);
    }
}
