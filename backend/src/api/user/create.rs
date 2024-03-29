use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;
use crate::user::user_response::UserResponse;
use actix_web::{
    http, post,
    web::{self, Json},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUserBody {
    username: String,
}

#[post("/user")]
pub async fn create_user(
    user: web::Json<NewUserBody>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<(Json<UserResponse>, http::StatusCode), ServerError> {
    let user = User::new(&auth_user.uid, &user.username, false)?;
    user.insert(&pool).await?;
    let user_response = UserResponse::from_uid(&user.uid, &pool).await?;
    Ok((Json(user_response), http::StatusCode::CREATED))
}

#[cfg(test)]
mod tests {
    use crate::{make_user, test::DBTest};
    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn user_creation(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let user = make_user!("black", &app);
        assert_eq!(user.uid, "black");
        assert_eq!(user.username, "black");
        let request_body = json!({
            "username": "white",
        });
        let resp = TestRequest::post()
            .uri("/api/user")
            .set_json(request_body)
            .insert_header(("x-authentication", "white"))
            .send_request(&app)
            .await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
