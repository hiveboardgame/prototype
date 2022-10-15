#![feature(is_some_with)]
use crate::bug::Bug;
use crate::color::Color;
use crate::history::History;
use crate::state::State;

mod state;
mod position;
mod bug;
mod color;
mod board;
mod piece;
mod player;
mod moves;
mod history;
mod hasher;

fn main() {
    let history = History::from_filepath("game.txt");
    let state = State::new_from_history(&history);
    println!("{}", state.board);
    println!("{} won!", state.board.winner().unwrap());
}
