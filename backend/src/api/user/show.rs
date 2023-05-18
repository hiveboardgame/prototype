use crate::db::util::DbPool;
use crate::model::user::User;
use crate::server_error::ServerError;
use actix_web::{get, web, HttpResponse};

#[get("/user/{uid}")]
pub async fn get_user(
    uid: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::find_by_uid(pool.get_ref(), uid.as_ref()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[cfg(test)]
mod tests {
    use crate::model::user::User;
    use crate::{make_user, test::DBTest};

    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn get_games(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);

        let req = TestRequest::get()
            .uri(&format!("/api/user/{}", black.uid))
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(user.username, black.username);
        assert_eq!(user.uid, black.uid);
        assert_eq!(user.is_guest, black.is_guest);
    }
}
