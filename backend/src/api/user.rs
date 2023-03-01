use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, Responder};
use serde::Deserialize;

use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;

#[get("/user/{uid}")]
pub async fn get_user(uid: web::Path<String>, pool: web::Data<DbPool>) -> impl Responder {
    User::find_by_uid(pool.get_ref(), uid.as_ref())
        .await
        .map(|user| web::Json(user))
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
    let user = User {
        uid: auth_user.uid,
        username: user.username.clone(),
        is_guest: false,
    };
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
    let user = User {
        uid: auth_user.uid,
        username: "Guest".to_string(), // TODO: random guest names
        is_guest: true,
    };
    user.insert(&pool)
        .await
        .map(|_| web::Json(user))
        .map_err(|err| ErrorInternalServerError(format!("couldn't create user: {:?}", err)))
    // TODO proper error handling
}
