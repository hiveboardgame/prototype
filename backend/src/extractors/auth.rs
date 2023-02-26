use alcoholic_jwt::{JWKS, Validation, validate, token_kid};
use std::future::Future;
use std::pin::Pin;
use actix_web::FromRequest;
use actix_web::error::{ Error, ErrorBadRequest, ErrorUnauthorized, ErrorInternalServerError };
use reqwest;

pub struct AuthenticatedUser {
    pub uid: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req_clone = req.clone(); // req is cheap to clone, and we must to avoid lifetime issues
        Box::pin(async move {
            let auth_token = req_clone.headers().get("X-Authentication")
                .ok_or(ErrorUnauthorized("Must include X-Authentication header"))?
                .to_str()
                .map_err(|err| ErrorBadRequest(format!("couldn't read X-Authentication header: {}", err)))?;
            let uid = validate_and_fetch_uid(auth_token).await?;
            Ok(AuthenticatedUser { uid })
        })
    }
}

async fn validate_and_fetch_uid(token: &str) -> Result<String, Error> {
    let authority = "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
    let jwks: JWKS = fetch_jwks(authority)
        .await
        .map_err(|err| ErrorInternalServerError(format!("failed to fetch JWKS: {}", err)))?;
    let validations = vec![Validation::Issuer(authority.to_string()), Validation::SubjectPresent];
    let kid = token_kid(&token)
        .map_err(|err| ErrorBadRequest(format!("failed to decode KID: {}", err)))?
        .ok_or(ErrorBadRequest("no KID in JWT"))?;
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    match validate(token, jwk, validations) {
        Ok(jwt) => match jwt.claims.get("sub") {
            Some(value) => Ok(value.to_string()),
            _ => Err(ErrorBadRequest("couldn't find subject in JWT claims")),
        },
        Err(err) => Err(ErrorUnauthorized(format!("invalid token: {}", err)))
    }
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn std::error::Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}
