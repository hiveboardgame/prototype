use actix_web::{
    get, http, post,
    web::{self, Json},
    HttpResponse,
};
use names::{Generator, Name};
use serde::Deserialize;

use crate::challenge::game_challenge_response::GameChallengeResponse;
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

#[cfg(test)]
mod tests {
    use crate::api::game::game_state_response::GameStateResponse;
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::{accept_challenge, make_challenge, make_guest_user, make_user, test::DBTest};

    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;
}
