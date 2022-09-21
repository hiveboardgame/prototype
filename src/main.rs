// use std::collections::{HashMap, HashSet};
// use std::fmt;
// use std::fmt::Write;

use crate::state::State;
use crate::position::Position;
use crate::bug::Bug;
use crate::color::Color;

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
        .spawn(&Position(0, 0), Bug::Queen, Color::Black, 1);
    println!("{}", state.board);
    for (i, pos) in state
        .board
        .neighbor_positions(&Position(0, 0))
        .iter()
        .enumerate()
    {
        state.board.spawn(pos, Bug::Ant, Color::Black, i as i8);
        println!("{}", state.board);
    }
}

//#[cfg(test)]
//mod tests {
//    use crate::Bug;
//    use crate::Color;
//    use crate::Position;
//    use crate::State;
//
//    #[test]
//    fn spawn() {
//        let mut state = State::new();
//        state
//            .board
//            .spawn(&Position(0, 0), Bug::Ant, Color::Black, 1);
//        let result = state.board.spawnable(Color::White, &Position(1, 0));
//        assert!(!result);
//    }
//}
