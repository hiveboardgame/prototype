use crate::{
    db::util::DbPool,
    model::{game::Game, user::User},
    server_error::ServerError,
};
use hive_lib::{
    bug::Bug, game_status::GameStatus, game_type::GameType, piece::Piece, position::Position,
    state::State, history::History,
};
use serde::Serialize;
use serde_with::serde_as;
use std::{collections::HashMap, str::FromStr};

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
    pub async fn new_from_db(game: &Game, pool: &DbPool) -> Result<Self, ServerError> {
        let history = History::new_from_str(game.history.clone())?;
        let state = State::new_from_history(&history)?;
        GameStateResponse::new_from(game, &state, pool).await
    }

    pub async fn new_from(game: &Game, state: &State, pool: &DbPool) -> Result<Self, ServerError> {
        Ok(Self {
            game_id: game.id,
            game_status: state.game_status.clone(),
            game_type: GameType::from_str(&game.game_type)?,
            tournament_queen_rule: state.tournament,
            turn: state.turn,
            white_player: User::find_by_uid(pool, &game.white_uid).await?,
            black_player: User::find_by_uid(pool, &game.black_uid).await?,
            moves: GameStateResponse::moves_as_string(state.board.moves(state.turn_color)),
            spawns: state
                .board
                .spawnable_positions(state.turn_color)
                .collect::<Vec<_>>(),
            reserve: state.board.reserve(state.turn_color, state.game_type),
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
