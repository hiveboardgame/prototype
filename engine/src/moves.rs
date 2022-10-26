use serde::Serialize;
use std::collections::HashMap;

use crate::{board::Board, bug::Bug, color::Color, piece::Piece, position::Position};

#[derive(Serialize, Clone)]
pub struct Moves<'board> {
    pub number: i32,
    pub color: Color,
    pub board: &'board Board,
    pub moves: HashMap<(Piece, Position), Vec<Position>>,
    pub spawnable_positions: Vec<Position>,
    pub reserve: HashMap<Bug, i8>,
}

impl<'board> Moves<'board> {
    pub fn new(number: i32, board: &'board Board) -> Self {
        let mut color = Color::White;
        if number.rem_euclid(2) == 1 {
            color = Color::Black;
        }
        Moves {
            number,
            color,
            board,
            moves: Moves::moves(color, board),
            spawnable_positions: Moves::spawnable_positions(&color, board),
            reserve: Moves::reserve(color, board),
        }
    }

    pub fn valid(
        &self,
        piece: &Piece,
        current_position: &Position,
        target_position: &Position,
    ) -> bool {
        return match self.moves.get(&(*piece, *current_position)) {
            None => false,
            Some(positions) => positions.contains(target_position),
        };
    }

    fn moves(color: Color, board: &Board) -> HashMap<(Piece, Position), Vec<Position>> {
        let mut moves: HashMap<(Piece, Position), Vec<Position>> = HashMap::new();
        // for all pieces on the board
        for pos in board.board.keys() {
            // that are the correct color
            if board.top_piece(pos).is_color(color) {
                // get all the moves
                for (start_pos, target_positions) in Bug::available_moves(pos, board) {
                    moves
                        .entry((board.top_piece(&start_pos), start_pos))
                        .or_default()
                        .append(&mut target_positions.clone());
                }
            }
        }
        moves
    }

    fn reserve(color: Color, board: &Board) -> HashMap<Bug, i8> {
        let mut bugs = Bug::bugs_count();
        for pieces in board.board.values() {
            for piece in pieces {
                if piece.is_color(color) {
                    if let Some(i) = bugs.get_mut(&piece.bug) {
                        *i -= 1;
                    }
                }
            }
        }
        bugs
    }

    fn spawnable_positions(color: &Color, board: &Board) -> Vec<Position> {
        board.spawnable_positions(color)
    }

    pub fn print_available_moves(&self, position: &Position) {
        let mut positions = self.board.board.keys().cloned().collect::<Vec<Position>>();
        let bug = format!(
            "*{}* ",
            self.board.board.get(position).unwrap().last().unwrap().bug
        );
        println!("Turn: {}", self.number);
        println!("Positions: {:?}", positions);
        println!("Moves: {:?}", self.moves);
        let piece = self.board.board.get(position).unwrap().last().unwrap();
        println!("Piece: {}", piece);
        println!("Position: {}", position);
        println!("Board: {}", self.board);
        positions.append(&mut self.moves.get(&(*piece, *position)).unwrap().clone());
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
        for (_i, y) in (min_y..=max_y).enumerate() {
            if y.rem_euclid(2) == 1 {
                s.push_str("  ")
            }
            for x in min_x..=max_x {
                match self.board.board.get(&Position(x, y)) {
                    Some(piece) => s.push_str(format!("{} ", piece.last().unwrap()).as_str()),
                    None => {
                        if self
                            .moves
                            .get(&(*piece, *position))
                            .unwrap()
                            .contains(&Position(x, y))
                        {
                            s.push_str(bug.as_str())
                        } else {
                            s.push_str("    ")
                        }
                    }
                };
            }
            s.push('\n');
        }
        println!("{s}");
    }
}
