use actix_web::{get, post, web, HttpResponse};
use names::{Generator, Name};
use serde::Deserialize;

use crate::api::game::challenge::GameChallengeResponse;
use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;

#[get("/user/{uid}")]
pub async fn get_user(
    uid: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::find_by_uid(pool.get_ref(), uid.as_ref()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[derive(Deserialize)]
pub struct NewUserBody {
    username: String,
}

fn random_guest_name() -> String {
    // we might consider storing the generator for (slightly) more efficient RNG
    let mut generator = Generator::with_naming(Name::Numbered);
    format!("guest-{}", generator.next().unwrap())
}

#[post("/user")]
pub async fn create_user(
    user: web::Json<NewUserBody>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::new(&auth_user.uid, &user.username, false)?;
    user.insert(&pool).await?;
    Ok(HttpResponse::Created().json(user))
}

#[post("/guest-user")]
pub async fn create_guest_user(
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::new(&auth_user.uid, &random_guest_name(), true)?;
    user.insert(&pool).await?;
    Ok(HttpResponse::Created().json(user))
}

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
    use crate::test::DBTest;
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    // If you need to inspect the response to debug something use this:
    // let body = test::read_body(resp).await;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_user(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let request_body = json!({
            "username": "black",
        });
        let resp = TestRequest::post()
            .uri("/api/user")
            .set_json(&request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "creating user failed");
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_guest_user(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let request_body = json!({
            "username": "guest",
        });
        let resp = TestRequest::post()
            .uri("/api/guest-user")
            .set_json(&request_body)
            .insert_header(("x-authentication", "guest"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "creating guest-user failed");
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_user_challenges(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let request_body = json!({
            "username": "black",
        });
        let resp = TestRequest::post()
            .uri("/api/user")
            .set_json(&request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "user creation failed");

        let resp = TestRequest::get()
            .uri("/api/user/black/challenges")
            .set_json(&request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "getting challenges failed");
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_user_games(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;

        let request_body = json!({
            "username": "black",
        });
        let resp = TestRequest::post()
            .uri("/api/user")
            .set_json(&request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "user creation failed");

        let resp = TestRequest::get()
            .uri("/api/user/black/games")
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "getting games failed");
    }
}
