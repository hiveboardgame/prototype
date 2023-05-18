use std::fmt::Display;
use std::str::FromStr;

use actix_web::{
    delete, get, post,
    web::{self, Json},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use hive_lib::game_error::GameError;
use hive_lib::game_type::GameType;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::util::DbPool,
    extractors::auth::AuthenticatedUser,
    game::game_state_response::GameStateResponse,
    model::{
        challenge::GameChallenge,
        game::{Game, NewGame},
        user::User,
    },
    server_error::ServerError,
};
use crate::api::game::challenge::game_challenge_response::GameChallengeResponse;

#[get("/game/challenge/{id}")]
pub async fn get_game_challenge(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<Json<GameChallengeResponse>, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    let response = GameChallengeResponse::from_model(&challenge, &pool).await?;
    Ok(web::Json(response))
}

