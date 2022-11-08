use hive_lib::{piece::Piece, position::Position, state::State};
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Clone, Default, Serialize, Deserialize, Store, PartialEq)]
pub struct GameStateStore {
    pub state: State,
    pub target_postitions: Vec<Position>,
    pub active: Option<Piece>,
    pub position: Option<Position>,
}

impl GameStateStore {
    pub fn reset(&mut self) {
        self.target_postitions = vec![];
        self.active = None;
        self.position = None;
    }

    pub fn spawn_active_piece(&mut self) {
        if let (Some(active), Some(position)) = (self.active, self.position) {
            self.state.play_turn(active, position);
        }
        self.reset()
    }

    pub fn show_moves(&mut self, piece: Piece, position: Position) {
        self.reset();
        let moves = self.state.board.moves(&self.state.turn_color);
        if let Some(positions) = moves.get(&(piece, position)) {
            self.target_postitions = positions.to_owned();
            self.active = Some(piece);
        }
    }

    pub fn show_spawns(&mut self, piece: Piece) {
        let spawns = self.state.board.spawnable_positions(&self.state.turn_color);
        self.target_postitions = spawns;
        self.active = Some(piece);
    }
}
