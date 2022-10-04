#![feature(is_some_with)]
use crate::state::State;
use crate::position::Position;
use crate::bug::Bug;
use crate::color::Color;
use crate::piece::Piece;
use crate::moves::Moves;
use crate::history::History;

mod state;
mod position;
mod bug;
mod color;
mod board;
mod piece;
mod player;
mod moves;
mod history;

fn main() {
    let mut state = State::new();
    state
        .board
        .spawn(&Position(0, 0), Piece::new(Bug::Ant, Color::White, 1));
    state
        .board
        .spawn(&Position(1, 0), Piece::new(Bug::Queen, Color::White, 1));
    state
        .board
        .spawn(&Position(2, 0), Piece::new(Bug::Grasshopper, Color::Black, 1));
    state
        .board
        .spawn(&Position(3, 0), Piece::new(Bug::Queen, Color::Black, 1));
    let moves = Moves::new(0, Color::White, &state.board);
    moves.print_available_moves(&Position(0, 0));
    let history = History::new();
    println!("{:?}", history);
}
