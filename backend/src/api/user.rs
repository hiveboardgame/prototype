use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, Responder};
use serde::Deserialize;

use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;

#[get("/user/{uid}")]
pub async fn get_user(uid: web::Path<String>, pool: web::Data<DbPool>) -> impl Responder {
    User::find_by_uid(pool.get_ref(), uid.as_ref())
        .await
        .map(web::Json)
        .map_err(|err| ErrorNotFound(format!("couldn't find user: {}", err)))
}

#[derive(Deserialize)]
pub struct NewUserBody {
    username: String,
}

#[post("/user")]
pub async fn create_user(
    user: web::Json<NewUserBody>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let user = User::new(&auth_user.uid, &user.username, false)
        .map_err(|err| ErrorBadRequest(format!("bad user fields: {:?}", err)))?;
    user.insert(&pool)
        .await
        .map(|_| web::Json(user))
        .map_err(|err| ErrorInternalServerError(format!("couldn't create user: {:?}", err)))
    // TODO proper error handling
}

#[post("/guest-user")]
pub async fn create_guest_user(
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> impl Responder {
    // TODO random guest names
    let user = User::new(&auth_user.uid, "Guest", true)
        .map_err(|err| ErrorBadRequest(format!("bad user fields: {:?}", err)))?;
    user.insert(&pool)
        .await
        .map(|_| web::Json(user))
        .map_err(|err| ErrorInternalServerError(format!("couldn't create user: {:?}", err)))
    // TODO proper error handling
}
