#![feature(is_some_with)]
use crate::state::State;
use crate::position::Position;
use crate::bug::Bug;
use crate::color::Color;
use crate::piece::Piece;
use crate::moves::Moves;

mod state;
mod position;
mod bug;
mod color;
mod board;
mod piece;
mod player;
mod moves;

fn main() {
    let mut state = State::new();
    state
        .board
        .spawn(&Position(0, 0), Piece::new(Bug::Ladybug, Color::White, 1));
    state
        .board
        .spawn(&Position(1, 0), Piece::new(Bug::Queen, Color::Black, 1));
    state
        .board
        .spawn(&Position(2, 0), Piece::new(Bug::Grasshopper, Color::Black, 1));
    let moves = Moves::new(0, Color::Black, &state.board);
    moves.print_moves(&Position(0, 0));
}
