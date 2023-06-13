use crate::extractors::auth::{AuthenticatedUser, AuthenticationError};
use crate::{
    api::game::game_state_response::GameStateResponse, db::util::DbPool, model::game::Game,
    server_error::ServerError,
};
use actix_web::{
    post,
    web::{self, Json, Path},
};
use hive_lib::{
    color::Color, game_control::GameControl, game_result::GameResult, game_status::GameStatus,
    game_type::GameType, history::History, position::Position, state::State,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::str::FromStr;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PlayRequest {
    Turn((String, String)),
    GameControl(GameControl),
}

#[post("/game/{id:\\d+}/play")]
pub async fn game_play(
    path: Path<i32>,
    play_request: Json<PlayRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<Json<GameStateResponse>, ServerError> {
    let game_id = path.into_inner();
    let game = Game::get(game_id, &pool).await?;
    if let GameStatus::Finished(_) = GameStatus::from_str(&game.game_status)? {
        Err(ServerError::UserInputError {
            field: format!("Can play: {play_request:?}"),
            reason: "Game is finished".to_string(),
        })?
    }
    let resp = match play_request.clone() {
        PlayRequest::Turn((piece, pos)) => {
            play_turn(&game, piece, pos, auth_user, pool.as_ref()).await
        }
        PlayRequest::GameControl(game_control) => {
            handle_game_control(&game, game_control, auth_user, pool.as_ref()).await
        }
    }?;
    Ok(web::Json(resp))
}

async fn play_turn(
    game: &Game,
    piece: String,
    pos: String,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    if game.turn % 2 == 0 {
        auth_user.authorize(&game.white_uid)?;
    } else {
        auth_user.authorize(&game.black_uid)?;
    }
    let history = History::new_from_str(game.history.clone())?;
    let mut state = State::new_from_history(&history)?;
    state.game_type = GameType::from_str(&game.game_type)?;
    let piece = piece.parse()?;
    let position = Position::from_string(&pos, &state.board)?;
    state.play_turn(piece, position)?;
    let board_move = format!("{piece} {pos}");
    let game = game
        .make_move(board_move, state.game_status.to_string(), pool)
        .await?;
    // TODO: handle game end, update rating
    GameStateResponse::new_from(&game, &state, pool).await
}

async fn handle_game_control(
    game: &Game,
    game_control: GameControl,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let auth_color = get_color(game, &auth_user)?;
    if !allowed_game_control(game, game_control.clone())? {
        Err(ServerError::UserInputError {
            field: format!("{game_control}"),
            reason: "Not allowed".to_string(),
        })?
    }
    if !request_color_matches(auth_color, game_control.clone()) {
        Err(ServerError::UserInputError {
            field: "game_control".to_string(),
            reason: "game control color and user color don't match".to_string(),
        })?
    }
    if !fresh_game_control(game, game_control.clone())? {
        Err(ServerError::UserInputError {
            field: "game_control".to_string(),
            reason: "game control already seen".to_string(),
        })?
    }
    match game_control {
        GameControl::Abort(_) => handle_abort(game, game_control, pool).await,
        GameControl::Resign(_) => handle_resign(game, game_control, auth_user, pool).await,
        GameControl::DrawOffer(_) => handle_draw_offer(game, game_control, pool).await,
        GameControl::DrawAccept(_) => handle_draw_accept(game, game_control, pool).await,
        GameControl::DrawReject(_) => handle_draw_reject(game, game_control, pool).await,
        GameControl::TakebackRequest(_) => handle_takeback_request(game, game_control, pool).await,
        GameControl::TakebackAccept(_) => handle_takeback_accept(game, game_control, pool).await,
        GameControl::TakebackReject(_) => handle_takeback_reject(game, game_control, pool).await,
    }
}

fn get_color(game: &Game, auth_user: &AuthenticatedUser) -> Result<Color, ServerError> {
    if auth_user.authorize(&game.white_uid).is_ok() {
        return Ok(Color::White);
    }
    if auth_user.authorize(&game.black_uid).is_ok() {
        return Ok(Color::Black);
    }
    Err(AuthenticationError::Forbidden)?
}

fn allowed_game_control(game: &Game, game_control: GameControl) -> Result<bool, ServerError> {
    match game_control {
        GameControl::Abort(_) => Ok(game.game_status == "NotStarted"),
        _ => Ok(game.game_status == "InProgress"),
    }
}

fn fresh_game_control(game: &Game, game_control: GameControl) -> Result<bool, ServerError> {
    // TODO: better handling of freshness
    if let Some(last) = last_game_control(game)? {
        return Ok(last != game_control);
    }
    Ok(true)
}

fn last_game_control(game: &Game) -> Result<Option<GameControl>, ServerError> {
    if let Some(last) = game.game_control_history.split_terminator(';').last() {
        if let Some(gc) = last.split(' ').last() {
            return Ok(Some(GameControl::from_str(gc)?));
        }
    }
    Ok(None)
}

fn ensure_turn_greater_zero(game: &Game, game_control: &GameControl) -> Result<(), ServerError> {
    if game.turn == 0 {
        Err(ServerError::UserInputError {
            field: format!("{game_control}"),
            reason: "Not not allowed on turn 0".to_string(),
        })?
    }
    Ok(())
}

fn ensure_game_control(game: &Game, current_game_control: GameControl) -> Result<(), ServerError> {
    let opposite_color = Color::from(current_game_control.color().opposite());
    let should_be_gc = match current_game_control {
        GameControl::TakebackAccept(_) => GameControl::TakebackRequest(opposite_color),
        GameControl::TakebackReject(_) => GameControl::TakebackRequest(opposite_color),
        GameControl::DrawReject(_) => GameControl::DrawOffer(opposite_color),
        GameControl::DrawAccept(_) => GameControl::DrawOffer(opposite_color),
        _ => unreachable!(),
    };
    if let Some(last_gc) = last_game_control(game)? {
        if last_gc == should_be_gc {
            return Ok(());
        }
    }
    Err(ServerError::UserInputError {
        field: format!("{current_game_control}"),
        reason: "Not allowed".to_string(),
    })?
}

async fn handle_draw_offer(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let game = game.write_game_control(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_draw_reject(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(game, game_control.clone())?;
    let game = game.write_game_control(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_draw_accept(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(game, game_control.clone())?;
    let game = game.accept_draw(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_resign(
    game: &Game,
    game_control: GameControl,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let winner_color = Color::from(get_color(game, &auth_user)?.opposite());
    let resigned_game = game
        .resign(
            game_control,
            GameStatus::Finished(GameResult::Winner(winner_color)),
            pool,
        )
        .await?;
    GameStateResponse::new_from_db(&resigned_game, pool).await
}

fn request_color_matches(color: Color, game_control: hive_lib::game_control::GameControl) -> bool {
    color == game_control.color()
}

async fn handle_abort(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let history = History::new_from_str(game.history.clone())?;
    let state = State::new_from_history(&history)?;
    let mut returned_game = (*game).clone();
    game.delete(pool).await?;
    returned_game
        .game_control_history
        .push_str(&format!("{}. {game_control};", state.turn));
    // WARN: this a bit hacky, we are returning a game that we just deleted...
    GameStateResponse::new_from(&returned_game, &state, pool).await
}

async fn handle_takeback_request(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_turn_greater_zero(game, &game_control)?;
    let game = game.write_game_control(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_takeback_accept(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(game, game_control.clone())?;
    let mut moves = game.history.split_terminator(';').collect::<Vec<_>>();
    moves.pop();
    let mut new_history = moves.join(";");
    new_history.push(';');
    let history = History::new_from_str(new_history.clone())?;
    let state = State::new_from_history(&history)?;
    let game = game
        .accept_takeback(
            new_history,
            state.game_status.to_string(),
            game_control,
            pool,
        )
        .await?;
    GameStateResponse::new_from(&game, &state, pool).await
}

async fn handle_takeback_reject(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(game, game_control.clone())?;
    let game = game.write_game_control(game_control, pool).await?;

    let history = History::new_from_str(game.history.clone())?;
    let state = State::new_from_history(&history)?;
    GameStateResponse::new_from(&game, &state, pool).await
}

#[cfg(test)]
mod tests {
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::{accept_challenge, game_control, make_challenge, make_user, play_turn};
    use crate::{api::game::game_state_response::GameStateResponse, test::DBTest};
    use actix_web::test::{self, TestRequest};
    use hive_lib::color::Color;
    use hive_lib::game_control::GameControl;
    use hive_lib::game_result::GameResult;
    use hive_lib::game_status::GameStatus;
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn resign_game(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(game.game_id, white.uid.clone(), "Resign", "White", &app);
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::Finished(hive_lib::game_result::GameResult::Winner(
                hive_lib::color::Color::Black
            ))
        );

        // Can't resign a finished game
        let request_body = json!({
            "GameControl": {"Resign": "Black" }
        });
        let resp = TestRequest::post()
            .uri(&format!("/api/game/{}/play", game.game_id))
            .set_json(&request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn draw_game(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(game.game_id, white.uid.clone(), "DrawOffer", "White", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawOffer(Color::White))
        );
        let game = game_control!(game.game_id, black.uid.clone(), "DrawReject", "Black", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawReject(Color::Black))
        );
        let game = game_control!(game.game_id, white.uid.clone(), "DrawOffer", "White", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawOffer(Color::White))
        );
        let game = game_control!(game.game_id, black.uid.clone(), "DrawAccept", "Black", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(2_i32, GameControl::DrawAccept(Color::Black))
        );
        assert_eq!(game.game_status, GameStatus::Finished(GameResult::Draw));

        // Can't play on a finished game
        let request_body = json!({
            "Turn": ["wQ", "-wL"],
        });
        let resp = TestRequest::post()
            .uri(&format!("/api/game/{}/play", game.game_id))
            .set_json(&request_body)
            .insert_header(("x-authentication", "white"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
    }

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn abort(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::NotStarted
        );
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = game_control!(game.game_id, black.uid.clone(), "Abort", "Black", &app);
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(game.turn as i32, GameControl::Abort(Color::Black))
        );
    }
    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn takeback_move(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game = play_turn!(game.game_id, white.uid.clone(), ["wL", "."], &app);
        assert_eq!(game.turn, 1);
        assert_eq!(game.history, vec![("wL".to_string(), ".".to_string())]);
        let game = play_turn!(game.game_id, black.uid.clone(), ["bL", "wL-"], &app);
        assert_eq!(game.turn, 2);
        let game = game_control!(
            game.game_id,
            white.uid.clone(),
            "TakebackRequest",
            "White",
            &app
        );
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(game.turn as i32, GameControl::TakebackRequest(Color::White))
        );
        let game = game_control!(
            game.game_id,
            black.uid.clone(),
            "TakebackReject",
            "Black",
            &app
        );
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(game.turn as i32, GameControl::TakebackReject(Color::Black))
        );
        let game = play_turn!(game.game_id, white.uid.clone(), ["wQ", "-wL"], &app);
        let game = game_control!(
            game.game_id,
            black.uid.clone(),
            "TakebackRequest",
            "Black",
            &app
        );
        let game = game_control!(
            game.game_id,
            white.uid.clone(),
            "TakebackAccept",
            "White",
            &app
        );
        assert_eq!(game.turn, 2);
        assert_eq!(
            game.history,
            vec![
                ("wL".to_string(), ".".to_string()),
                ("bL".to_string(), "wL-".to_string())
            ]
        );
        assert_eq!(
            game.game_control_history.last().unwrap(),
            &(
                (game.turn + 1) as i32,
                GameControl::TakebackAccept(Color::White)
            )
        );
    }
    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn play_game(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let mut game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let moves = vec![
            ["wL", "."],
            ["bL", "/wL"],
            ["wM", "wL/"],
            ["bQ", "bL\\"],
            ["wQ", "\\wL"],
            ["bP", "bQ\\"],
            ["wA1", "wM\\"],
            ["bA1", "-bL"],
            ["wM", "/bQ"],
            ["bA1", "\\wQ"],
            ["wB1", "/wM"],
            ["bB1", "bQ-"],
            ["wA1", "bB1-"],
            ["bB2", "bP\\"],
            ["wB1", "wM"],
            ["bA1", "wL\\"],
            ["wB1", "-bQ"],
        ];
        for (i, mov) in moves.iter().enumerate() {
            if i % 2 == 0 {
                game = play_turn!(game.game_id, white.uid.clone(), mov, &app);
            } else {
                game = play_turn!(game.game_id, black.uid.clone(), mov, &app);
            }
        }
        assert_eq!(
            game.game_status,
            hive_lib::game_status::GameStatus::Finished(hive_lib::game_result::GameResult::Winner(
                hive_lib::color::Color::White
            ))
        );
        assert_eq!(game.turn, 16);
        // Can't play on a finished game
        let request_body = json!({
            "Turn": ["bL", "-wQ"],
        });
        let resp = TestRequest::post()
            .uri(&format!("/api/game/{}/play", game.game_id))
            .set_json(&request_body)
            .insert_header(("x-authentication", "black"))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
    }
}
