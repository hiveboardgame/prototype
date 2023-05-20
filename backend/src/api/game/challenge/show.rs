use crate::api::game::challenge::game_challenge_response::GameChallengeResponse;
use crate::{db::util::DbPool, model::challenge::GameChallenge, server_error::ServerError};
use actix_web::{
    get,
    web::{self, Json},
};
use uuid::Uuid;

#[get("/game/challenge/{id}")]
pub async fn get_game_challenge(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<Json<GameChallengeResponse>, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    let response = GameChallengeResponse::from_model(&challenge, &pool).await?;
    Ok(web::Json(response))
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
    async fn get_game_challenge(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let white = make_user!("white", &app);
        let challenge = make_challenge!(white.uid.clone(), "White", &app);
        let req = TestRequest::get()
            .uri(&format!("/api/game/challenge/{}", challenge.id))
            .to_request();
        let challenge_from_endpoint: GameChallengeResponse =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(challenge_from_endpoint.id, challenge.id);
    }
}
