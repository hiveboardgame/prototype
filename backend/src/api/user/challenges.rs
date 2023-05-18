use actix_web::{
    get,
    web::{self},
    HttpResponse,
};

use crate::challenge::game_challenge_response::GameChallengeResponse;
use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;

#[get("/user/{uid}/challenges")]
pub async fn get_user_challenges(
    uid: web::Path<String>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    auth_user.authorize(&uid)?;
    let user = User::find_by_uid(pool.get_ref(), uid.as_ref()).await?;
    let mut response: Vec<GameChallengeResponse> = Vec::new();
    for challenge in &user.get_challenges(&pool).await? {
        response.push(GameChallengeResponse::from_model_with_user(
            challenge,
            user.clone(),
        )?);
    }
    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {

    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::{make_challenge, make_user, test::DBTest};

    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn user_challenges(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);

        let req = TestRequest::get()
            .uri("/api/user/black/challenges")
            .insert_header(("x-authentication", "black"))
            .to_request();
        let challenges: Vec<GameChallengeResponse> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(challenges.len(), 0);

        make_challenge!(black.uid.clone(), "Random", &app);
        let req = TestRequest::get()
            .uri("/api/user/black/challenges")
            .insert_header(("x-authentication", "black"))
            .to_request();
        let challenges: Vec<GameChallengeResponse> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(challenges.len(), 1);
    }
}
