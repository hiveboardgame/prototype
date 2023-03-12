use crate::model::{game_request::GameRequest, game_response::GameResponse};
use crate::server_error::ServerError;
use actix_web::web::{self, Json, Path};
use actix_web::post;
use hive_lib::{game_error::GameError, history::History, position::Position, state::State};

fn get_game_state_from_db(_game_id: u64) -> Result<State, GameError> {
    let game = "game.txt";
    let history = History::from_filepath(game)?;
    State::new_from_history(&history)
}

fn play_turn(mut state: State, piece: String, pos: String) -> Result<GameResponse, ServerError> {
    let board_move = format!("{} {}", piece, pos);
    let piece = piece.parse()?;
    let pos = Position::from_string(&pos, &state.board)?;
    state.play_turn(piece, pos)?;
    let game = "game.txt";
    state.history.write_move(game, state.turn, board_move);
    Ok(GameResponse::new_from_state(&state))
}

#[post("/game/{id:\\d+}/play")]
pub async fn game_play(
    path: Path<u64>,
    game_request: Json<GameRequest>,
) -> Result<Json<GameResponse>, ServerError> {
    let game_id = path.into_inner();
    let game_request: GameRequest = game_request.clone(); // This is hacky!
    let state = get_game_state_from_db(game_id)?;
    let resp = match game_request {
        GameRequest::Turn((piece, pos)) => play_turn(state, piece, pos),
        GameRequest::GameControl(any) => {
            println!("{} to be implemented", any);
            return Err(ServerError::Unimplemented);
        }
    }?;
    Ok(web::Json(resp))
}
