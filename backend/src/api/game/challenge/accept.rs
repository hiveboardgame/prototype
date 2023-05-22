use crate::{
    db::util::DbPool,
    extractors::auth::AuthenticatedUser,
    game::challenge::game_challenge_response::ChallengeError,
    game::game_state_response::GameStateResponse,
    model::{
        challenge::GameChallenge,
        game::{Game, NewGame},
    },
    server_error::ServerError,
};
use actix_web::{
    post,
    web::{self, Json},
};
use uuid::Uuid;

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

#[cfg(test)]
mod tests {
    use crate::challenge::{
        accept::GameStateResponse, game_challenge_response::GameChallengeResponse,
    };
    use crate::test::DBTest;
    use crate::{accept_challenge, make_challenge, make_user};
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn accept_game_challenge(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let white = make_user!("white", &app);
        let black = make_user!("black", &app);
        let open_challenge = make_challenge!(white.uid.clone(), "White", &app);
        let accepted_challenge = accept_challenge!(open_challenge.id, black.uid.clone(), &app);
        let resp = TestRequest::get()
            .uri(&format!("/api/game/challenge/{}", open_challenge.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        assert_eq!(accepted_challenge.game_id, 1);
    }
}
