use hive_lib::{
    bug::Bug, game_status::GameStatus, game_type::GameType, piece::Piece, position::Position,
    state::State,
};
use serde::Serialize;
use serde_with::serde_as;
use std::collections::HashMap;
use crate::{model::{user::User, game::Game}, db::util::DbPool, server_error::ServerError};

#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct GameStateResponse {
    game_id: i32,
    turn: usize,
    game_status: GameStatus,
    game_type: GameType,
    tournament_queen_rule: bool,
    white_player: User,
    black_player: User,
    #[serde_as(as = "Vec<(_, _)>")]
    moves: HashMap<String, Vec<Position>>,
    spawns: Vec<Position>,
    reserve: HashMap<Bug, i8>,
    history: Vec<(String, String)>,
}

impl GameStateResponse {
    pub async fn new_from(game: &Game, state: &State, pool: &DbPool) -> Result<Self, ServerError> {
        Ok(Self {
            game_id: game.id,
            game_status: state.game_status.clone(),
            game_type: state.game_type,
            tournament_queen_rule: state.tournament,
            turn: state.turn,
            white_player: User::find_by_uid(&pool, &game.white_uid).await?,
            black_player: User::find_by_uid(&pool, &game.black_uid).await?,
            moves: GameStateResponse::moves_as_string(state.board.moves(&state.turn_color)),
            spawns: state.board.spawnable_positions(&state.turn_color),
            reserve: state.board.reserve(&state.turn_color, state.game_type),
            history: state.history.moves.clone(),
        })
    }

    fn moves_as_string(
        moves: HashMap<(Piece, Position), Vec<Position>>,
    ) -> HashMap<String, Vec<Position>> {
        let mut mapped = HashMap::new();
        for ((piece, _pos), possible_pos) in moves.into_iter() {
            mapped.insert(piece.to_string(), possible_pos);
        }
        mapped
    }
}
