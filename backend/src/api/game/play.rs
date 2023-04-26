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
    game_control::GameControl, game_type::GameType, history::History, position::Position,
    state::State,
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
    pool: &DbPool,
) -> Result<GameStateResponse, ServerError> {
    let game = Game::get(game_id, pool).await?;
    let history = History::new_from_str(game.history.clone())?;
    let mut state = State::new_from_history(&history)?;
    state.game_type = GameType::from_str(&game.game_type)?;
    let piece = piece.parse()?;
    let position = Position::from_string(&pos, &state.board)?;
    state.play_turn(piece, position)?;
    let board_move = format!("{piece} {pos}");
    game.make_move(board_move, pool).await?;
    println!("state.game_status is {}", state.game_status);
    println!("game.game_status is {}", game.game_status);
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
    let game = Game::get(game_id, &pool).await?;
    if game.turn % 2 == 0 {
        auth_user.authorize(&game.white_uid)?;
    } else {
        auth_user.authorize(&game.black_uid)?;
    }
    let resp = match play_request.clone() {
        PlayRequest::Turn((piece, pos)) => play_turn(game_id, piece, pos, pool.as_ref()),
        PlayRequest::GameControl(any) => {
            println!("{} to be implemented", any);
            return Err(ServerError::Unimplemented);
        }
    }
    .await?;
    Ok(web::Json(resp))
}
