use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use crate::bug::Bug;
use crate::color::Color;
use crate::piece::Piece;
use crate::position::Position;

pub struct Board {
    board: HashMap<Position, Vec<Piece>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut positions = self.board.keys().cloned().collect::<Vec<Position>>();
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
        for y in min_y..=max_y {
            if y.rem_euclid(2) == 1 {
                write!(s, "{}", "  ")?;
            }
            for x in min_x..=max_x {
                match self.board.get(&Position(x, y)) {
                    Some(piece) => write!(s, "{} ", piece.last().unwrap())?,
                    None => write!(s, "{}", "    ")?,
                };
            }
            write!(s, "{}", "\n")?;
        }
        write!(f, "{}", s)
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: HashMap::new(),
        }
    }

    pub fn positions_around(&self, position: &Position) -> Vec<Position> {
        return vec![
            Position(position.0 - 1, position.1 - 1), // North West
            Position(position.0, position.1 - 1),     // North East
            Position(position.0 + 1, position.1),     // East
            Position(position.0, position.1 + 1),     // South East
            Position(position.0 - 1, position.1 + 1), // South West
            Position(position.0 - 1, position.1),     // West
        ];
    }

    pub fn positions_taken_around(&self, position: &Position) -> Vec<Position> {
        return self
            .positions_around(position)
            .into_iter()
            .filter(|pos| self.board.contains_key(pos))
            .collect();
    }

    pub fn neighbors(&self, position: &Position) -> Vec<Vec<Piece>> {
        return self
            .positions_around(&position)
            .iter()
            .filter_map(|pos| self.board.get(&pos))
            .cloned()
            .collect();
    }

    pub fn queen_played(&self, color: Color) -> bool {
        return self
            .board
            .values()
            .any(|p| p.contains(&Piece::new(Bug::Queen, color, 1)));
    }

    fn walk_board(
        &self,
        position: Position,
        excluded_position: &Position,
        mut visited: HashSet<Position>,
    ) -> HashSet<Position> {
        visited.insert(position);
        for pos in self.positions_taken_around(&position).iter() {
            if pos != excluded_position && !visited.contains(pos) {
                visited.extend(&self.walk_board(*pos, excluded_position, visited.clone()));
            }
        }
        return visited;
    }

    pub fn move_splits_hive(&self, position: &Position) -> bool {
        let len = self.board.keys().len();
        let mut visited = HashSet::new();
        match self
            .board
            .keys()
            .filter(|p| *p != position)
            .cloned()
            .collect::<Vec<Position>>()
            .pop()
        {
            Some(start) => visited = self.walk_board(start.clone(), position, visited.clone()),
            None => return false,
        }
        return visited.len() < (len - 1);
    }

    pub fn top_layer_neighbors(&self, position: &Position) -> Vec<Piece> {
        return self
            .positions_around(&position)
            .iter()
            .filter_map(|pos| self.board.get(&pos).and_then(|v| v.last()))
            .cloned()
            .collect();
    }

    pub fn negative_space(&self) -> Vec<Position> {
        let taken = self.board.keys().cloned().collect::<HashSet<Position>>();
        let mut all_neighbors = HashSet::new();
        for pos in taken.iter() {
            for pos in self.positions_around(pos) {
                all_neighbors.insert(pos);
            }
        }
        all_neighbors
            .difference(&taken)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn spawnable(&self, color: Color, position: &Position) -> bool {
        if self.board.keys().len() < 2 {
            return true;
        }
        !self
            .top_layer_neighbors(position)
            .iter()
            .map(|piece| piece.color)
            .collect::<Vec<Color>>()
            .contains(&color.opposite())
    }

    pub fn spawn(&mut self, position: &Position, piece: Piece) {
        self.board
            .entry(position.clone())
            .and_modify(|v| v.push(piece))
            .or_insert(vec![piece]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests_positions_taken_around() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        let pos = board.positions_taken_around(&Position(0, 0));
        assert_eq!(pos, vec![Position(1, 0)]);
    }

    #[test]
    fn tests_neighbors() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        let mut pieces = vec![Piece::new(Bug::Ant, Color::Black, 1)];
        board.spawn(&Position(1, 0), pieces.last().unwrap().clone());
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        pieces.push(Piece::new(Bug::Beetle, Color::Black, 1));
        board.spawn(&Position(1, 0), pieces.last().unwrap().clone());
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        board.spawn(&Position(0, 2), Piece::new(Bug::Ladybug, Color::Black, 1));
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces]);
    }

    #[test]
    fn tests_negative_space() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        let mut positions = board.positions_around(&Position(0, 0));
        let mut negative_space = board.negative_space();
        assert_eq!(negative_space.sort(), positions.sort());
    }

    #[test]
    fn tests_walk_board() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, 3));
        let excluded = Position(5, 0);
        let visited = board.walk_board(Position(0, 0), &excluded, HashSet::new());
        assert_eq!(visited.len(), 4);
        let excluded = Position(2, 0);
        let visited = board.walk_board(Position(0, 0), &excluded, HashSet::new());
        assert_eq!(visited.len(), 2);
        let visited = board.walk_board(Position(0, 0), &excluded, HashSet::new());
        assert_eq!(visited.len(), 2);
        let visited = board.walk_board(Position(1, 0), &excluded, HashSet::new());
        assert_eq!(visited.len(), 2);
        let visited = board.walk_board(Position(3, 0), &excluded, HashSet::new());
        assert_eq!(visited.len(), 1);

        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Ant, Color::Black, 5));
        }
        for pos in board.positions_around(&Position(0, 0)).iter() {
            let visited = board.walk_board(Position(3, 0), pos, HashSet::new());
            if pos == &Position(1,0) {
                assert_eq!(visited.len(), 2);
            } else {
                assert_eq!(visited.len(), 8);
            }
        }
    }

    #[test]
    fn tests_move_splits_hive() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, 3));
        assert!(!board.move_splits_hive(&Position(0, 0)));
        assert!(board.move_splits_hive(&Position(1, 0)));
        assert!(board.move_splits_hive(&Position(2, 0)));
        assert!(!board.move_splits_hive(&Position(3, 0)));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Ant, Color::Black, 5));
        }
        for pos in board.positions_around(&Position(0, 0)).iter() {
            if pos == &Position(1, 0) {
                assert!(board.move_splits_hive(pos));
            } else {
                assert!(!board.move_splits_hive(pos));
            };
        }
    }
}
