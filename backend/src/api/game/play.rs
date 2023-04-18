use crate::{
    api::game::game_state_response::GameStateResponse,
    db::util::DbPool, model::game::Game, server_error::ServerError,
};
use actix_web::web::{self, Json, Path};
use actix_web::post;
use hive_lib::{
    game_control::GameControl, history::History, position::Position,
    state::State,
};
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

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
    let piece = piece.parse()?;
    let pos = Position::from_string(&pos, &state.board)?;
    state.play_turn(piece, pos)?;
    GameStateResponse::new_from(&game, &state, pool).await
}

#[post("/game/{id:\\d+}/play")]
pub async fn game_play(
    path: Path<i32>,
    play_request: Json<PlayRequest>,
    pool: web::Data<DbPool>,
) -> Result<Json<GameStateResponse>, ServerError> {
    let game_id = path.into_inner();
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
