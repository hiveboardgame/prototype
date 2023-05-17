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

#[get("/game/lobby")]
pub async fn get_lobby_challenges(pool: web::Data<DbPool>) -> Result<HttpResponse, ServerError> {
    let challenges = GameChallenge::get_public(&pool).await?;
    let mut responses = Vec::new();
    // TODO: batch all users into one query
    for challenge in challenges {
        responses.push(GameChallengeResponse::from_model(&challenge, &pool).await?);
    }
    Ok(HttpResponse::Ok().json(responses))
}

#[post("/game/challenge")]
pub async fn create_game_challenge(
    game: web::Json<NewGameChallengeRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::create(&auth_user, &game, &pool).await?;
    let response = GameChallengeResponse::from_model(&challenge, &pool).await?;
    Ok(HttpResponse::Created().json(response))
}

#[get("/game/challenge/{id}")]
pub async fn get_game_challenge(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    let response = GameChallengeResponse::from_model(&challenge, &pool).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/game/challenge/{id}/accept")]
pub async fn accept_game_challenge(
    id: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<Json<GameStateResponse>, ServerError> {
    let challenge = GameChallenge::get(&id, &pool).await?;
    if challenge.challenger_uid == auth_user.uid {
        return Err(ChallengeError::OwnChallenge.into());
    }
    let (white_uid, black_uid) = match challenge.color_choice.to_lowercase().as_str() {
        "black" => (auth_user.uid, challenge.challenger_uid.clone()),
        "white" => (challenge.challenger_uid.clone(), auth_user.uid),
        _ => {
            if rand::random() {
                (challenge.challenger_uid.clone(), auth_user.uid)
            } else {
                (auth_user.uid, challenge.challenger_uid.clone())
            }
        }
    };
    let new_game = NewGame {
        black_uid,
        game_status: "NotStarted".to_string(),
        game_type: challenge.game_type.clone(),
        history: String::new(),
        game_control_history: String::new(),
        tournament_queen_rule: challenge.tournament_queen_rule,
        turn: 0,
        white_uid,
        ranked: challenge.ranked,
    };
    let game = Game::create(&new_game, &pool).await?;
    challenge.delete(&pool).await?;
    let resp = GameStateResponse::new_from_db(&game, &pool).await?;
    Ok(web::Json(resp))
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

#[cfg(test)]
mod tests {
    use crate::challenge::GameChallengeResponse;
    use crate::{accept_challenge, make_challenge, make_user};
    use crate::{api::game::game_state_response::GameStateResponse, test::DBTest};
    use actix_web::test::{self, TestRequest};
    use hive_lib::game_status::GameStatus;
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_challenge(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        assert_eq!(game.game_status, GameStatus::NotStarted);
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn test_lobby(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        make_challenge!(white.uid.clone(), "White", &app);
        make_challenge!(black.uid.clone(), "Black", &app);
        let resp = TestRequest::get()
            .uri("/api/game/lobby")
            .send_request(&app)
            .await;
        assert!(
            resp.status().is_success(),
            "getting lobby challenges failed"
        );
        let body = test::read_body(resp).await;
        //TODO Fix my very hacky workaround
        assert_eq!(body.len(), 497);
    }
}
