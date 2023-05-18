
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

fn random_guest_name() -> String {
    // we might consider storing the generator for (slightly) more efficient RNG
    let mut generator = Generator::with_naming(Name::Numbered);
    format!("guest-{}", generator.next().unwrap())
}

#[post("/guest-user")]
pub async fn create_guest_user(
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<(Json<User>, http::StatusCode), ServerError> {
    let user = User::new(&auth_user.uid, &random_guest_name(), true)?;
    user.insert(&pool).await?;
    Ok((Json(user), http::StatusCode::CREATED))
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

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn guest_user_creation(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        make_guest_user!("guest", &app);
    }

}
