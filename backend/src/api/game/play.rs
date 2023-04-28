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

fn get_uid(game: &Game, auth_user: AuthenticatedUser) -> Result<String, ServerError> {
    if auth_user.authorize(&game.white_uid).is_ok() {
        return Ok(game.white_uid.clone());
    }
    if auth_user.authorize(&game.black_uid).is_ok() {
        return Ok(game.black_uid.clone());
    }
    Err(AuthenticationError::Forbidden)?
}

fn get_color(game: &Game, auth_user: AuthenticatedUser) -> Result<Color, ServerError> {
    if auth_user.authorize(&game.white_uid).is_ok() {
        return Ok(Color::White);
    }
    if auth_user.authorize(&game.black_uid).is_ok() {
        return Ok(Color::Black);
    }
    Err(AuthenticationError::Forbidden)?
}

async fn play_turn(
    game_id: i32,
    piece: String,
    pos: String,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let mut game = Game::get(game_id, pool).await?;
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
    game.make_move(board_move, pool).await?;
    if state.game_status.to_string() != game.game_status {
        game.set_status(state.game_status.clone(), pool).await?;
    }
    GameStateResponse::new_from(&game, &state, pool).await
}

#[post("/game/{id:\\d+}/play")]
pub async fn game_play(
    path: Path<i32>,
    play_request: Json<PlayRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<Json<GameStateResponse>, ServerError> {
    let game_id = path.into_inner();
    // TODO: make sure the game isn't finished
    let resp = match play_request.clone() {
        PlayRequest::Turn((piece, pos)) => {
            play_turn(game_id, piece, pos, auth_user, pool.as_ref()).await
        }
        PlayRequest::GameControl(game_control) => {
            handle_game_control(game_id, game_control, auth_user, pool.as_ref()).await
        }
    }?;
    Ok(web::Json(resp))
}

async fn handle_game_control(
    game_id: i32,
    game_control: GameControl,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    if !allowed_game_control(game_id, pool, game_control.clone()).await? {
        Err(ServerError::UserInputError {
            field: format!("{game_control}"),
            reason: "Not allowed".to_string(),
        })?
    }
    if !request_color_matches(game_id, auth_user.clone(), game_control.clone(), pool).await? {
        Err(ServerError::UserInputError {
            field: "game_control".to_string(),
            reason: "game control color and user color don't match".to_string(),
        })?
    }
    match game_control {
        GameControl::Abort(_) => handle_abort(game_id, pool).await,
        GameControl::Resign(_) => handle_resign(game_id, auth_user, pool).await,
        GameControl::DrawOffer(_) => handle_draw_offer(game_id, auth_user, pool).await,
        _ => unimplemented!(),
    }
}

// InProgress - all but abort
// NotStarted - Abort
// Change NotStarted to second move
// Finished no game controls
async fn allowed_game_control(
    game_id: i32,
    pool: &DbPool,
    game_control: GameControl,
) -> Result<bool, ServerError> {
    let game = Game::get(game_id, pool).await?;
    match game_control {
        GameControl::Abort(_) => Ok(game.game_status == "NotStarted"),
        _ => Ok(game.game_status == "InProgress"),
    }
}

async fn handle_draw_offer(
    game_id: i32,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let game = Game::get(game_id, pool).await?;
    let color = get_color(&game, auth_user)?;
    let history = History::new_from_str(game.history.clone())?;
    let state = State::new_from_history(&history)?;
    GameStateResponse::new_from(&game, &state, pool).await
}

async fn handle_resign(
    game_id: i32,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let game = Game::get(game_id, pool).await?;
    let winner_color = Color::from(get_color(&game, auth_user)?.opposite());
    let game = Game::get(game_id, pool).await?;
    let history = History::new_from_str(game.history.clone())?;

    let mut state = State::new_from_history(&history)?;
    state.game_status = GameStatus::Finished(GameResult::Winner(winner_color));
    if state.game_status.to_string() != game.game_status {
        game.set_status(state.game_status.clone(), pool).await?;
    }
    GameStateResponse::new_from(&game, &state, pool).await
}

async fn request_color_matches(
    game_id: i32,
    auth_user: AuthenticatedUser,
    game_control: hive_lib::game_control::GameControl,
    pool: &DbPool,
) -> Result<bool, ServerError> {
    let game = Game::get(game_id, pool).await?;
    let color = get_color(&game, auth_user)?;
    Ok(color == game_control.color())
}

async fn handle_abort(game_id: i32, pool: &DbPool) -> Result<GameStateResponse, ServerError> {
    let game = Game::get(game_id, pool).await?;
    let history = History::new_from_str(game.history.clone())?;
    let state = State::new_from_history(&history)?;
    game.delete(pool).await?;
    // WARN: this a bit hacky, we are returning a game that we just deleted...
    GameStateResponse::new_from(&game, &state, pool).await
}
