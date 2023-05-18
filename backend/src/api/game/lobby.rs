use actix_web::{
    get,
    web::{self, Json},
};

use crate::api::game::challenge::game_challenge_response::GameChallengeResponse;
use crate::db::util::DbPool;

use crate::model::challenge::GameChallenge;
use crate::server_error::ServerError;

#[get("/game/lobby")]
pub async fn get_lobby_challenges(
    pool: web::Data<DbPool>,
) -> Result<Json<Vec<GameChallengeResponse>>, ServerError> {
    let challenges = GameChallenge::get_public(&pool).await?;
    let mut resp = Vec::new();
    // TODO: batch all users into one query
    for challenge in challenges {
        resp.push(GameChallengeResponse::from_model(&challenge, &pool).await?);
    }
    Ok(web::Json(resp))
}

#[cfg(test)]
mod tests {
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::test::DBTest;
    use crate::{make_challenge, make_user};
    use actix_web::test::{self, TestRequest};

    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_lobby(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        make_challenge!(white.uid.clone(), "White", &app);
        make_challenge!(black.uid.clone(), "Black", &app);
        let req = TestRequest::get().uri("/api/game/lobby").to_request();
        let game_challenge_responses: Vec<GameChallengeResponse> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(game_challenge_responses.len(), 2);
    }
}
