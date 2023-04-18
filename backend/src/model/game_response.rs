use hive_lib::{
    bug::Bug, game_status::GameStatus, game_type::GameType, piece::Piece, position::Position,
    state::State,
};
use serde::Serialize;
use serde_with::serde_as;
use std::collections::HashMap;
use crate::model::user::User;

#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct GameResponse {
    game_id: u64,
    turn: usize,
    game_status: GameStatus,
    game_type: GameType,
    tournament: bool,
    white_player: User,
    black_player: User,
    #[serde_as(as = "Vec<(_, _)>")]
    moves: HashMap<String, Vec<Position>>,
    spawns: Vec<Position>,
    reserve: HashMap<Bug, i8>,
    history: Vec<(String, String)>,
}

impl GameResponse {
    // TODO we don't need to pass everything in once we have a couple of good helpers :)
    pub fn new_from_state(state: &State) -> Self {
        let white_player = User::new("1", "white", false).unwrap();
        let black_player = User::new("2", "black", false).unwrap();
        Self {
            game_id: state.game_id,
            game_status: state.game_status.clone(),
            game_type: state.game_type,
            tournament: state.tournament,
            turn: state.turn,
            white_player,
            black_player,
            moves: GameResponse::moves_as_string(state.board.moves(state.turn_color)),
            spawns: state.board.spawnable_positions(state.turn_color).collect::<Vec<_>>(),
            reserve: state.board.reserve(state.turn_color, state.game_type),
            history: state.history.moves.clone(),
        }
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
