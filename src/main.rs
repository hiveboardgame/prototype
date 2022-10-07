#![feature(is_some_with)]
use crate::board::Board;
use crate::bug::Bug;
use crate::color::Color;
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
    let history = History::new();
    println!("{:?}", history);
    let board = Board::new_from_history(&history);
    println!("{}", board);
    println!("{} won!", board.winner().unwrap());
}
