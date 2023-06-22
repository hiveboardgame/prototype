use crate::{
    api::game::challenge::game_challenge_response::ChallengeError,
    extractors::auth::AuthenticationError,
};
use actix_web::{
    error::{QueryPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use diesel::result::Error as DieselError;
use hive_lib::game_error::GameError;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Authentication error: {0}")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("invalid field {field}: {reason}")]
    UserInputError { field: String, reason: String },
    #[error("Hive game error: {0}")]
    GameError(#[from] GameError),
    #[error("Internal hive game error: {0}")]
    InternalGameError(GameError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Challenge error: {0}")]
    ChallengeError(#[from] ChallengeError),
    #[error("Missing Query parm error: {0}")]
    QueryParamError(#[from] QueryPayloadError),

    #[error("Unimplemented")]
    Unimplemented,
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::GameError(_) => StatusCode::BAD_REQUEST,
            Self::InternalGameError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthenticationError(err) => match err {
                AuthenticationError::MissingToken => StatusCode::UNAUTHORIZED,
                AuthenticationError::Forbidden => StatusCode::FORBIDDEN,
                AuthenticationError::MalformedJWT(_) | AuthenticationError::MissingSubject => {
                    StatusCode::BAD_REQUEST
                }
                AuthenticationError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthenticationError::InvalidJWT(_) => StatusCode::UNAUTHORIZED,
            },
            Self::UserInputError {
                field: _,
                reason: _,
            } => StatusCode::BAD_REQUEST,
            Self::DatabaseError(err) => match err {
                DieselError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::ChallengeError(err) => match err {
                ChallengeError::MissingChallenger(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ChallengeError::OwnChallenge => StatusCode::BAD_REQUEST,
            },
            Self::QueryParamError(_) => StatusCode::BAD_REQUEST,
            Self::Unimplemented => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        // TODO: don't send a message for 500s
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}
