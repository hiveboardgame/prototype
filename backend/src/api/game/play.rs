use crate::extractors::auth::AuthenticatedUser;

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

async fn play_turn(
    game_id: i32,
    piece: String,
    pos: String,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let game = Game::get(game_id, pool).await?;
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
    // TODO make sure the game isn't finished
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
    match game_control {
        GameControl::Resign => handle_resign(game_id, auth_user, pool).await,
        _ => unimplemented!(),
    }
}

async fn handle_resign(
    game_id: i32,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let game = Game::get(game_id, pool).await?;
    let mut winner_color: Option<Color> = None;
    if auth_user.authorize(&game.white_uid).is_ok() {
        winner_color = Some(Color::Black);
    }
    if auth_user.authorize(&game.black_uid).is_ok() {
        winner_color = Some(Color::White);
    }
    if winner_color.is_none() {
        auth_user.authorize(&game.black_uid)?
    }
    let history = History::new_from_str(game.history.clone())?;
    let mut state = State::new_from_history(&history)?;
    state.game_status = GameStatus::Finished(GameResult::Winner(winner_color.unwrap()));
    if state.game_status.to_string() != game.game_status {
        game.set_status(state.game_status.clone(), pool).await?;
    }
    GameStateResponse::new_from(&game, &state, pool).await
}
