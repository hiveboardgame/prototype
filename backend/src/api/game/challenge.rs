use std::fmt::Display;

use actix_web::{delete, get, post, web, HttpResponse};
use hive_lib::game_type::GameType;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::model::challenge::GameChallenge;
use crate::model::user::User;
use crate::server_error::ServerError;
use crate::{db::util::DbPool, extractors::auth::AuthenticatedUser};

#[derive(Error, Debug)]
pub enum ChallengeError {
    #[error("Couldn't find challenge creator (uid {0})")]
    MissingChallenger(String),
    #[error("Players can't accept their own challenges!")]
    OwnChallenge,
}

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
#[serde(rename_all = "camelCase")]
pub struct NewGameChallengeRequest {
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
#[serde(rename_all = "camelCase")]
pub struct GameChallengeWithChallenger {
    challenge: GameChallenge,
    challenger: User,
}

#[post("/game/challenge")]
pub async fn create_game_challenge(
    game: web::Json<NewGameChallengeRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::create(&auth_user, &game, &pool).await?;
    Ok(HttpResponse::Created().json(challenge))
}

#[get("/game/challenge/{id}")]
pub async fn get_game_challenge(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    let challenger = match challenge.get_challenger(&pool).await {
        Ok(challenger) => challenger,
        Err(diesel::result::Error::NotFound) => {
            let uid = challenge.challenger_uid.clone();
            return Err(ChallengeError::MissingChallenger(uid).into());
        }
        Err(err) => return Err(err.into()),
    };
    Ok(HttpResponse::Ok().json(GameChallengeWithChallenger {
        challenge,
        challenger,
    }))
}

#[post("/game/challenge/{id}/accept")]
pub async fn accept_game_challenge(
    id: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    if challenge.challenger_uid == auth_user.uid {
        return Err(ChallengeError::OwnChallenge.into());
    }
    // TODO: delete the challenge, create a new game between auth_user and the
    // challenger, and return the newly created game
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
