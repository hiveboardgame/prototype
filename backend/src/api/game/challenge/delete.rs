use crate::{
    db::util::DbPool, extractors::auth::AuthenticatedUser, model::challenge::GameChallenge,
    server_error::ServerError,
};
use actix_web::{
    delete,
    web::{self},
    HttpResponse,
};
use uuid::Uuid;

#[delete("/game/challenge/{id}")]
pub async fn delete_game_challenge(
    id: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    auth_user.authorize(&challenge.challenger_uid)?;
    challenge.delete(&pool).await?;
    Ok(HttpResponse::NoContent().finish())
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
    async fn delete_game_challenge(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let white = make_user!("white", &app);
        let open_challenge = make_challenge!(white.uid.clone(), "White", &app);
        let req_delete = TestRequest::delete()
            .uri(&format!("/api/game/challenge/{}", open_challenge.id))
            .insert_header(("x-authentication", white.uid.clone()))
            .send_request(&app)
            .await;
        assert!(req_delete.status().is_success());
        let resp = TestRequest::get()
            .uri(&format!("/api/game/challenge/{}", open_challenge.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
    }
}
