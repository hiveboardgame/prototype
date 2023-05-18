use actix_web::{
    delete,
    web::{self},
    HttpResponse,
};

use uuid::Uuid;

use crate::{
    db::util::DbPool, extractors::auth::AuthenticatedUser, model::challenge::GameChallenge,
    server_error::ServerError,
};

#[delete("/game/challenge/{id}")]
pub async fn delete_game_challenge(
    id: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    auth_user.authorize(&challenge.challenger_uid)?;
    challenge.delete(&pool).await?;
    Ok(HttpResponse::NoContent().finish())
}
