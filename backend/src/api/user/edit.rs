use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;
use crate::user::user_response::UserResponse;
use actix_web::{
    http, put,
    web::{self, Json},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EditedUserBody {
    username: String,
}

#[put("/user/")]
pub async fn edit_user(
    auth_user: AuthenticatedUser,
    edited_user: web::Json<EditedUserBody>,
    pool: web::Data<DbPool>,
) -> Result<(Json<UserResponse>, http::StatusCode), ServerError> {
    let user = User::find_by_uid(&auth_user.uid, &pool).await?;
    user.update_username(&edited_user.username, &pool)
        .await
        .map_err(|_| ServerError::UserInputError {
            field: "username".to_string(),
            reason: "already exists".to_string(),
        })?;
    let user_response = UserResponse::from_uid(&auth_user.uid, &pool).await?;
    Ok((Json(user_response), http::StatusCode::CREATED))
}

#[cfg(test)]
mod tests {
    use crate::user::user_response::UserResponse;
    use crate::{make_user, test::DBTest};
    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn edit_user(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let _black = make_user!("black", &app);
        let _white = make_user!("white", &app);
        let request_body = json!({
            "username": "white",
        });
        let resp = TestRequest::put()
            .uri("/api/user/")
            .set_json(request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let request_body = json!({
            "username": "Iongler"
        });

        let req = TestRequest::put()
            .uri("/api/user/")
            .set_json(request_body)
            .insert_header(("x-authentication", "black"))
            .to_request();
        let user: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(user.username, "Iongler");
    }
}
