use actix_web::FromRequest;
use alcoholic_jwt::{token_kid, validate, Validation, ValidationError, JWKS};
use reqwest;
use std::future::Future;
use std::pin::Pin;
use thiserror::Error;

const FIREBASE_JWT_AUTHORITY: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

use crate::config::ServerConfig;
use crate::server_error::ServerError;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("user did not provide an X-Authorization header")]
    MissingToken,
    #[error("user with not authorized for resource")]
    Forbidden,
    #[error("JWT in X-Authorization header was malformed: {0}")]
    MalformedJWT(String),
    #[error("JWT in X-Authorization was invalid: {0}")]
    InvalidJWT(#[from] ValidationError),
    #[error("JWT is missing valid subject string")]
    MissingSubject,
    #[error("internal error: {0}")]
    InternalError(String),
}

pub struct AuthenticatedUser {
    pub uid: String,
}

impl AuthenticatedUser {
    pub fn authorize(&self, expected_uid: &str) -> Result<(), AuthenticationError> {
        if self.uid == expected_uid {
            Ok(())
        } else {
            Err(AuthenticationError::Forbidden)
        }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = ServerError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req_clone = req.clone(); // req is cheap to clone, and we must to avoid lifetime issues
        Box::pin(async move {
            let config = req_clone
                .app_data::<ServerConfig>()
                .expect("couldn't retrieve server config");
            let auth_token = req_clone
                .headers()
                .get("X-Authentication")
                .ok_or(AuthenticationError::MissingToken)?
                .to_str()
                .map_err(|err| {
                    AuthenticationError::MalformedJWT(format!(
                        "couldn't read X-Authentication header: {}",
                        err
                    ))
                })?;
            let uid = validate_and_fetch_uid(auth_token, config).await?;
            Ok(AuthenticatedUser { uid })
        })
    }
}

// TODO: cache google's cert more intelligently
async fn validate_and_fetch_uid(
    token: &str,
    config: &ServerConfig,
) -> Result<String, AuthenticationError> {
    let jwks: JWKS = fetch_jwks().await.map_err(|err| {
        AuthenticationError::InternalError(format!("failed to fetch JWKS: {}", err))
    })?;
    let validations = vec![
        Validation::Issuer(config.firebase_jwt_issuer.to_string()),
        Validation::SubjectPresent,
    ];
    let kid = token_kid(token)?.ok_or(AuthenticationError::MalformedJWT(
        "no KID in JWT".to_string(),
    ))?;
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let jwt = validate(token, jwk, validations)?;
    let subject = jwt
        .claims
        .get("sub")
        .ok_or(AuthenticationError::MissingSubject)?
        .as_str()
        .ok_or(AuthenticationError::MissingSubject)?;
    Ok(subject.to_string())
}

async fn fetch_jwks() -> Result<JWKS, Box<dyn std::error::Error>> {
    let res = reqwest::get(FIREBASE_JWT_AUTHORITY).await?;
    let val = res.json::<JWKS>().await?;
    Ok(val)
}
