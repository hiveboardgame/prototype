use hive_lib::{state::State, position::Position, piece::Piece};
use yewdux::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Clone, Default, Serialize, Deserialize, Store, PartialEq)]
pub struct GameStateStore {
    pub state: State,
    pub target_postitions: Vec<Position>,
    pub active: Option<Piece>,
    pub position: Option<Position>,
}
