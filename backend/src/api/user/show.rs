use crate::db::util::DbPool;
use crate::server_error::ServerError;
use crate::user::user_response::UserResponse;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserParams {
    username: String,
    //exists: Option<String>, Considering this in order to just get a bool in case user exists
}

#[get("/user/{uid}")]
pub async fn get_user(
    uid: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user_response = UserResponse::from_uid(&uid, &pool).await?;
    Ok(HttpResponse::Ok().json(user_response))
}

#[get("/user")]
pub async fn get_user_by_params(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let params = web::Query::<UserParams>::from_query(req.query_string())?;
    let user_response = UserResponse::from_username(&params.username, &pool).await?;
    Ok(HttpResponse::Ok().json(user_response))
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
    async fn get_user_by_params(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let req = TestRequest::get()
            .uri(&format!("/api/user?username=black"))
            .to_request();
        let user: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(user.username, black.username);
        assert_eq!(user.uid, black.uid);
        assert_eq!(user.is_guest, black.is_guest);
        assert_eq!(user.played, 0);
        assert_eq!(user.rating, 1500);
    }
}
