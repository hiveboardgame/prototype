use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;
use crate::user::user_response::UserResponse;
use actix_web::{
    http, post,
    web::{self, Json},
};
use names::{Generator, Name};

fn random_guest_name() -> String {
    // we might consider storing the generator for (slightly) more efficient RNG
    let mut generator = Generator::with_naming(Name::Numbered);
    format!("guest-{}", generator.next().unwrap())
}

#[post("/guest-user")]
pub async fn create_guest_user(
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<(Json<UserResponse>, http::StatusCode), ServerError> {
    let user = User::new(&auth_user.uid, &random_guest_name(), true)?;
    user.insert(&pool).await?;
    let user_response = UserResponse::from_uid(&user.uid, &pool).await?;
    Ok((Json(user_response), http::StatusCode::CREATED))
}

#[cfg(test)]
mod tests {
    use crate::{make_guest_user, test::DBTest};
    use actix_web::test::{self, TestRequest};
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
