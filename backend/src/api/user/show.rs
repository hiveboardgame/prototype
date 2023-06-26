use crate::db::util::DbPool;
use crate::server_error::ServerError;
use crate::user::user_response::UserResponse;
use actix_web::{get, web, web::Json, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserSearch {
    uid: Option<String>,
    username: Option<String>,
}

#[get("/user/{uid}")]
pub async fn get_user(
    uid: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user_response = UserResponse::from_uid(&uid, &pool).await?;
    Ok(HttpResponse::Ok().json(user_response))
}

#[get("/user/")]
pub async fn search_user(
    search_request: Json<UserSearch>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    if search_request.uid.is_some() {
        let user_response =
            UserResponse::from_uid(&search_request.uid.clone().unwrap(), &pool).await?;
        return Ok(HttpResponse::Ok().json(user_response));
    } else if search_request.username.is_some() {
        let user_response =
            UserResponse::from_username(&search_request.username.clone().unwrap(), &pool).await?;
        return Ok(HttpResponse::Ok().json(user_response));
    }
    Err(ServerError::UserInputError {
        field: ("JSON".to_string()),
        reason: ("Missing fields uid or username".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use crate::api::user::user_response::UserResponse;
    use crate::{make_user, test::DBTest};
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn get_user(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let req = TestRequest::get()
            .uri(&format!("/api/user/{}", black.uid))
            .to_request();
        let user: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(user.username, black.username);
        assert_eq!(user.uid, black.uid);
        assert_eq!(user.is_guest, black.is_guest);
        assert_eq!(user.played, 0);
        assert_eq!(user.rating, 1500);
    }
    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn search_user(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let request_body = json!({ "username": "black" });
        let req = TestRequest::get()
            .uri("/api/user/")
            .set_json(&request_body)
            .to_request();
        let user: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(user.username, black.username);
        assert_eq!(user.uid, black.uid);
        assert_eq!(user.is_guest, black.is_guest);
        assert_eq!(user.played, 0);
        assert_eq!(user.rating, 1500);
        let request_body = json!({ "uid": "white" });
        let req = TestRequest::get()
            .uri("/api/user/")
            .set_json(&request_body)
            .to_request();
        let user: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(user.username, white.username);
        assert_eq!(user.uid, white.uid);
        assert_eq!(user.is_guest, white.is_guest);
        assert_eq!(user.played, 0);
        assert_eq!(user.rating, 1500);

        let request_body = json!({});
        let resp = TestRequest::get()
            .uri("/api/user/")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
    }
}
