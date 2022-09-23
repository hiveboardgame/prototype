// use std::collections::{HashMap, HashSet};
// use std::fmt;
// use std::fmt::Write;

use crate::state::State;
use crate::position::Position;
use crate::bug::Bug;
use crate::color::Color;
use crate::piece::Piece;

mod state;
mod position;
mod bug;
mod color;
mod board;
mod piece;
mod player;

fn main() {
    let mut state = State::new();
    state
        .board
        .spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
    println!("{}", state.board);
    for (i, pos) in state
        .board
        .positions_around(&Position(0, 0))
        .iter()
        .enumerate()
    {
        state.board.spawn(pos, Piece::new(Bug::Ant, Color::Black, i as i8));
        println!("{}", state.board);
    }
}
