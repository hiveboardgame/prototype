use actix_web::{post, web::Form, web::Json};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[post("/token")]
pub async fn token(
    body: Form<TokenBody>,
) -> Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>>
{
    let req = body.into_inner();
    println!("{:?}", req);
    let client = BasicClient::new(
        ClientId::new(".apps.googleusercontent.com".to_string()),
        Some(ClientSecret::new("xxx".to_string())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).expect("AuthUrl::new failed"),
        Some(TokenUrl::new( "https://oauth2.googleapis.com/token".to_string()).expect("TokenUrl::new failed")),
    )
    .set_redirect_uri(RedirectUrl::new(req.redirect_uri).expect("Issue constructing Redirect url"));


    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Err(err) => {
            panic!("{:?}", err.to_string());
        }
        Ok(val) => {
            println!("Tokens received from OAuth provider!");
            Json(val)
        }
    }
}
