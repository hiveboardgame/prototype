use actix_web::error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::FromRequest;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use reqwest;
use std::future::Future;
use std::pin::Pin;

use crate::config::ServerConfig;

pub struct AuthenticatedUser {
    pub uid: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
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
                .ok_or(ErrorUnauthorized("Must include X-Authentication header"))?
                .to_str()
                .map_err(|err| {
                    ErrorBadRequest(format!("couldn't read X-Authentication header: {}", err))
                })?;
            let uid = validate_and_fetch_uid(auth_token, &config).await?;
            Ok(AuthenticatedUser { uid })
        })
    }
}

// TODO: cache google's cert more intelligently
async fn validate_and_fetch_uid(token: &str, config: &ServerConfig) -> Result<String, Error> {
    let jwks: JWKS = fetch_jwks(&config.firebase_jwt_authority)
        .await
        .map_err(|err| ErrorInternalServerError(format!("failed to fetch JWKS: {}", err)))?;
    let validations = vec![
        Validation::Issuer(config.firebase_jwt_issuer.to_string()),
        Validation::SubjectPresent,
    ];
    let kid = token_kid(&token)
        .map_err(|err| ErrorBadRequest(format!("failed to decode KID: {}", err)))?
        .ok_or(ErrorBadRequest("no KID in JWT"))?;
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    validate(token, jwk, validations)
        .map_err(|err| ErrorUnauthorized(format!("invalid token: {}", err)))
        .and_then(|jwt| jwt.claims.get("sub").ok_or(ErrorBadRequest("couldn't find subject in JWT claims"))
            .and_then(|subject| subject.as_str().ok_or(ErrorBadRequest("JWT subject must be a string")))
            .and_then(|token| Ok(token.to_owned())))
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn std::error::Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}
