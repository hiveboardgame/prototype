use crate::{
    db::util::DbPool,
    model::{game::Game, ratings::Rating, user::User},
    server_error::ServerError,
};
use hive_lib::{
    bug::Bug, color::Color, game_control::GameControl, game_status::GameStatus,
    game_status::GameStatus::Finished, game_type::GameType, history::History, piece::Piece,
    position::Position, state::State,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{collections::HashMap, str::FromStr};

#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateResponse {
    pub game_id: i32,
    pub turn: usize,
    pub game_status: GameStatus,
    pub game_type: GameType,
    pub tournament_queen_rule: bool,
    pub white_player: User,
    pub black_player: User,
    #[serde_as(as = "Vec<(_, _)>")]
    pub moves: HashMap<String, Vec<Position>>,
    pub spawns: Vec<Position>,
    pub reserve_black: HashMap<Bug, i8>,
    pub reserve_white: HashMap<Bug, i8>,
    pub history: Vec<(String, String)>,
    pub game_control_history: Vec<(i32, GameControl)>,
    pub white_rating: Option<f64>,
    pub black_rating: Option<f64>,
    pub white_rating_change: Option<f64>,
    pub black_rating_change: Option<f64>,
}

impl GameStateResponse {
    pub async fn new_from_db(game: &Game, pool: &DbPool) -> Result<Self, ServerError> {
        let history = History::new_from_str(game.history.clone())?;
        let state = State::new_from_history(&history)?;
        GameStateResponse::new_from(game, &state, pool).await
    }

    pub async fn new_from(game: &Game, state: &State, pool: &DbPool) -> Result<Self, ServerError> {
        let (white_rating, black_rating, white_rating_change, black_rating_change) = {
            if let Finished(_) = GameStatus::from_str(&game.game_status).unwrap() {
                (
                    game.white_rating,
                    game.black_rating,
                    game.white_rating_change,
                    game.black_rating_change,
                )
            } else {
                (
                    Some(Rating::for_uid(&game.white_uid, pool).await?.rating),
                    Some(Rating::for_uid(&game.black_uid, pool).await?.rating),
                    None,
                    None,
                )
            }
        };
        Ok(Self {
            game_id: game.id,
            game_status: GameStatus::from_str(&game.game_status)?,
            game_type: GameType::from_str(&game.game_type)?,
            tournament_queen_rule: state.tournament,
            turn: state.turn,
            white_player: User::find_by_uid(&game.white_uid, pool).await?,
            black_player: User::find_by_uid(&game.black_uid, pool).await?,
            moves: GameStateResponse::moves_as_string(state.board.moves(state.turn_color)),
            spawns: state
                .board
                .spawnable_positions(state.turn_color)
                .collect::<Vec<_>>(),
            reserve_black: state
                .board
                .reserve(Color::Black, game.game_type.parse().unwrap()),
            reserve_white: state
                .board
                .reserve(Color::White, game.game_type.parse().unwrap()),
            history: state.history.moves.clone(),
            game_control_history: Self::gc_history(&game.game_control_history),
            white_rating,
            black_rating,
            white_rating_change,
            black_rating_change,
        })
    }

    fn gc_history(gcs: &str) -> Vec<(i32, GameControl)> {
        let mut ret = Vec::new();
        for gc_str in gcs.split_terminator(';') {
            let turn: i32;
            let gc: GameControl;
            // TODO: This code is janky
            if let Some(turn_str) = gc_str.split(' ').next() {
                turn = turn_str.strip_suffix('.').unwrap().parse().unwrap();
                if let Some(gc_token) = gc_str.split(' ').nth(1) {
                    gc = gc_token.parse().unwrap();
                    ret.push((turn, gc));
                }
            }
        }
        ret
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
