use std::fmt::Display;

use actix_web::{delete, post, web, HttpResponse};
use hive_lib::game_type::GameType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::challenge::GameChallenge;
use crate::server_error::ServerError;
use crate::{db::util::DbPool, extractors::auth::AuthenticatedUser};

#[derive(Deserialize, Debug)]
pub enum ColorChoice {
    White,
    Black,
    Random,
}

impl Display for ColorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "White"),
            Self::Black => write!(f, "Black"),
            Self::Random => write!(f, "Random"),
        }
    }
}

#[derive(Deserialize)]
pub struct NewChallengeRequest {
    // Whether this challenge should be listed publicly
    pub public: bool,

    // Whether the game will be ranked
    pub ranked: bool,

    // Whether the game follows the "tournament" rules, i.e. the queen
    // cannot be played first.
    pub tournament_queen_rule: bool,

    // The challenger's color choice
    pub color_choice: ColorChoice,

    pub game_type: GameType,
}

#[derive(Serialize)]
pub struct NewChallengeResponse {
    challenge_url: String,
}

#[post("/game/challenge")]
pub async fn create_game_challenge(
    game: web::Json<NewChallengeRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::create(&auth_user, &game, &pool).await?;
    let challenge_url = format!("/game/challenge/{}", challenge.id);
    Ok(HttpResponse::Created().json(NewChallengeResponse { challenge_url }))
}

#[post("/game/challenge/{id}/accept")]
pub async fn accept_game_challenge(
    id: web::Path<Uuid>,
    _auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let _challenge = GameChallenge::get(&id, &pool).await?;
    // TODO: create a new game between auth_user and the challenger, then
    // delete the challenge
    Err(ServerError::Unimplemented)
}

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
