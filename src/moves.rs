use std::collections::HashMap;

use crate::{board::Board, bug::Bug, color::Color, piece::Piece, position::Position};

pub struct Moves<'board> {
    number: i32,
    color: Color,
    board: &'board Board,
    moves: HashMap<(Piece, Position), Vec<Position>>,
    spawnable_positions: Vec<Position>,
    spawnable_bugs: Vec<Bug>,
    reserve: HashMap<Bug, i8>,
}

impl<'board> Moves<'board> {
    pub fn new(number: i32, color: Color, board: &'board Board) -> Self {
        Moves {
            number,
            color,
            board,
            moves: Moves::moves(color, board),
            spawnable_bugs: Moves::spawnable_bugs(color, board),
            spawnable_positions: Moves::spawnable_positions(color, board),
            reserve: Moves::reserve(color, board),
        }
    }

    fn moves(color: Color, board: &Board) -> HashMap<(Piece, Position), Vec<Position>> {
        return HashMap::new();
    }

    fn spawnable_bugs(color: Color, board: &Board) -> Vec<Bug> {
        return Vec::new();
    }

    fn spawnable_positions(color: Color, board: &Board) -> Vec<Position> {
        return Vec::new();
    }

    fn reserve(color: Color, board: &Board) -> HashMap<Bug, i8> {
        return HashMap::new();
    }

    pub fn print_moves(&self, position: &Position) {
        let mut positions = self.board.board.keys().cloned().collect::<Vec<Position>>();
        let bug = format!(
            "*{}* ",
            self.board.board.get(position).unwrap().last().unwrap().bug
        );
        let moves = Bug::available_moves(&position, self.board);
        println!("Turn: {}", self.number);
        println!("Positions: {:?}", positions);
        println!("Moves: {:?}", moves);
        println!("Board:");
        positions.append(&mut moves.get(position).unwrap().clone());
        positions.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let min_x = positions
            .iter()
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or(&Position(0, 0))
            .0;

        let max_x = positions
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or(&Position(0, 0))
            .0;

        let min_y = positions
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(&Position(0, 0))
            .1;

        let max_y = positions
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(&Position(0, 0))
            .1;

        let mut s = "".to_string();
        for (i, y) in (min_y..=max_y).enumerate() {
            if y.rem_euclid(2) == 1 {
                s.push_str("  ")
            }
            for x in min_x..=max_x {
                match self.board.board.get(&Position(x, y)) {
                    Some(piece) => s.push_str(format!("{} ", piece.last().unwrap()).as_str()),
                    None => {
                        if moves.contains_key(&Position(x, y)) {
                            s.push_str(bug.as_str())
                        } else {
                            s.push_str("    ")
                        }
                    }
                };
            }
            s.push_str("\n");
        }
        println!("{s}");
    }
}
