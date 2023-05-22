use crate::{
    db::util::DbPool,
    model::{challenge::GameChallenge, user::User},
    server_error::ServerError,
};
use chrono::{DateTime, Utc};
use hive_lib::game_error::GameError;
use hive_lib::game_type::GameType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ChallengeError {
    #[error("Couldn't find challenge creator (uid {0})")]
    MissingChallenger(String),
    #[error("Players can't accept their own challenges!")]
    OwnChallenge,
}

#[derive(Deserialize, Serialize, Debug)]
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

impl FromStr for ColorChoice {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "White" => Ok(ColorChoice::White),
            "Black" => Ok(ColorChoice::Black),
            "Random" => Ok(ColorChoice::Random),
            _ => Err(GameError::ParsingError {
                found: s.to_string(),
                typ: "color choice string".to_string(),
            }),
        }
    }
}

#[derive(Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameChallengeResponse {
    pub id: Uuid,
    pub challenger: User,
    pub game_type: GameType,
    pub ranked: bool,
    pub public: bool,
    pub tournament_queen_rule: bool,
    pub color_choice: ColorChoice,
    pub created_at: DateTime<Utc>,
}

impl GameChallengeResponse {
    pub async fn from_model(challenge: &GameChallenge, pool: &DbPool) -> Result<Self, ServerError> {
        let challenger = match challenge.get_challenger(pool).await {
            Ok(challenger) => challenger,
            Err(diesel::result::Error::NotFound) => {
                let uid = challenge.challenger_uid.clone();
                return Err(ChallengeError::MissingChallenger(uid).into());
            }
            Err(err) => return Err(err.into()),
        };
        GameChallengeResponse::from_model_with_user(challenge, challenger)
    }

    pub fn from_model_with_user(
        challenge: &GameChallenge,
        challenger: User,
    ) -> Result<Self, ServerError> {
        let game_type: GameType = challenge
            .game_type
            .parse()
            .map_err(ServerError::InternalGameError)?;
        let color_choice: ColorChoice = challenge
            .color_choice
            .parse()
            .map_err(ServerError::InternalGameError)?;
        Ok(GameChallengeResponse {
            id: challenge.id,
            challenger,
            game_type,
            ranked: challenge.ranked,
            public: challenge.public,
            tournament_queen_rule: challenge.tournament_queen_rule,
            color_choice,
            created_at: challenge.created_at,
        })
    }
}
