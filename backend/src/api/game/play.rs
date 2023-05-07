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
            println!("GC req");
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
    game.make_move(board_move, state.game_status.to_string(), pool)
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
    let auth_color = get_color(&game, &auth_user)?;
    if !allowed_game_control(&game, game_control.clone())? {
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
    if !fresh_game_control(&game, game_control.clone())? {
        Err(ServerError::UserInputError {
            field: "game_control".to_string(),
            reason: "game control already seen".to_string(),
        })?
    }
    match game_control {
        GameControl::Abort(_) => handle_abort(game, pool).await,
        GameControl::Resign(_) => handle_resign(game, auth_user, pool).await,
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
    if let Some(last) = last_game_control(game)? {
        return Ok(last != game_control);
    }
    Ok(true)
}

fn last_game_control(game: &Game) -> Result<Option<GameControl>, ServerError> {
    if let Some(last) = game.game_control_history.split_terminator(";").last() {
        println!("Last game control is: {}", last);
        if let Some(gc) = last.split(" ").last() {
            println!("game control part is is: {}", gc);
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
    if let Some(last_gc) = last_game_control(&game)? {
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
    game.write_game_control(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_draw_reject(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(&game, game_control.clone())?;
    game.write_game_control(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_draw_accept(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(&game, game_control.clone())?;
    game.accept_draw(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_resign(
    game: &Game,
    auth_user: AuthenticatedUser,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let winner_color = Color::from(get_color(&game, &auth_user)?.opposite());
    game.set_status(GameStatus::Finished(GameResult::Winner(winner_color)), pool)
        .await?;
    GameStateResponse::new_from_db(&game, pool).await
}

fn request_color_matches(color: Color, game_control: hive_lib::game_control::GameControl) -> bool {
    color == game_control.color()
}

async fn handle_abort(game: &Game, pool: &DbPool) -> Result<GameStateResponse, ServerError> {
    let history = History::new_from_str(game.history.clone())?;
    let state = State::new_from_history(&history)?;
    game.delete(pool).await?;
    // WARN: this a bit hacky, we are returning a game that we just deleted...
    GameStateResponse::new_from(&game, &state, pool).await
}

async fn handle_takeback_request(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_turn_greater_zero(game, &game_control)?;
    game.write_game_control(game_control, pool).await?;
    GameStateResponse::new_from_db(&game, pool).await
}

async fn handle_takeback_accept(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(&game, game_control.clone())?;
    let mut moves = game.history.split_terminator(";").collect::<Vec<_>>();
    moves.pop();
    let mut new_history = moves.join(";");
    new_history.push_str(";");
    let history = History::new_from_str(new_history.clone())?;
    let state = State::new_from_history(&history)?;
    game.accept_takeback(new_history, state.game_status.to_string(), game_control, pool)
        .await?;
    GameStateResponse::new_from(&game, &state, pool).await
}

async fn handle_takeback_reject(
    game: &Game,
    game_control: GameControl,
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    ensure_game_control(&game, game_control.clone())?;
    game.write_game_control(game_control, pool).await?;

    let history = History::new_from_str(game.history.clone())?;
    let state = State::new_from_history(&history)?;
    GameStateResponse::new_from(&game, &state, pool).await
}
