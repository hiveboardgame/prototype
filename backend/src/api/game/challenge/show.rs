use actix_web::{
    get,
    web::{self, Json},
};

use uuid::Uuid;

use crate::api::game::challenge::game_challenge_response::GameChallengeResponse;
use crate::{db::util::DbPool, model::challenge::GameChallenge, server_error::ServerError};

#[get("/game/challenge/{id}")]
pub async fn get_game_challenge(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<Json<GameChallengeResponse>, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    let response = GameChallengeResponse::from_model(&challenge, &pool).await?;
    Ok(web::Json(response))
}
