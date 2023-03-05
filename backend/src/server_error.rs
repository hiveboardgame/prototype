use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use hive_lib::game_error::GameError;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Error, Serialize)]
pub enum ServerError {
    #[error("Access forbidden")]
    Forbidden,
    #[error("Not a valid game move: {message}")]
    GameError { message: String },
    #[error("Not found")]
    NotFound,
    #[error("Unknown internal error")]
    Unknown,
}

impl ServerError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "NotFound".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
            Self::Unknown => "Unknown".to_string(),
            Self::GameError { message: _ } => "Game Error".to_string(),
        }
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::GameError { message: _ } => StatusCode::BAD_REQUEST,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<GameError> for ServerError {
    fn from(err: GameError) -> ServerError {
        ServerError::GameError {
            message: { err.to_string() },
        }
    }
}
