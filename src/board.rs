use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use crate::bug::Bug;
use crate::color::Color;
use crate::piece::Piece;
use crate::position::{Direction, Position};

// TODO: make this a newtype?
#[derive(Clone)]
pub struct Board {
    pub board: HashMap<Position, Vec<Piece>>,
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
                write!(s, "  ")?;
            }
            for x in min_x..=max_x {
                match self.board.get(&Position(x, y)) {
                    Some(piece) => write!(s, "{} ", piece.last().unwrap())?,
                    None => write!(s, "    ")?,
                };
            }
            writeln!(s)?;
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

    pub fn positions_for_color(&self, color: Color) -> Vec<Position> {
        self.board
            .keys()
            .filter(|pos| {
                self.board
                    .get(pos)
                    .unwrap() // unwrap here is safe because we are got pos from board.keys
                    .last()
                    .expect(format!("Could not find a piece at pos: {}", pos).as_str())
                    .is_color(color)
            })
            .cloned()
            .collect()
    }

    pub fn positions_around(&self, position: &Position) -> Vec<Position> {
        vec![
            position.to(&Direction::NW),
            position.to(&Direction::NE),
            position.to(&Direction::E),
            position.to(&Direction::SE),
            position.to(&Direction::SW),
            position.to(&Direction::W),
        ]
    }

    pub fn level(&self, position: &Position) -> usize {
        return match self.board.get(position) {
            None => 0,
            Some(pieces) => pieces.len(),
        };
    }

    pub fn top_piece(&self, position: &Position) -> Piece {
        self.board.get(position).unwrap().last().unwrap().clone()
    }

    pub fn gated(&self, level: usize, from: &Position, to: &Position) -> bool {
        let (pos1, pos2) = from.common_adjacent_positions(to);
        self.board.get(&pos1).is_some_and(|v| v.len() >= level)
            && self.board.get(&pos2).is_some_and(|v| v.len() >= level)
    }

    pub fn positions_taken_around(&self, position: &Position) -> Vec<Position> {
        self.positions_around(position)
            .into_iter()
            .filter(|pos| self.board.contains_key(pos))
            .collect()
    }

    pub fn positions_available_around(&self, position: &Position) -> Vec<Position> {
        self.positions_around(position)
            .into_iter()
            .filter(|pos| !self.board.contains_key(pos))
            .collect()
    }

    pub fn neighbors(&self, position: &Position) -> Vec<Vec<Piece>> {
        return self
            .positions_around(position)
            .iter()
            .filter_map(|pos| self.board.get(pos))
            .cloned()
            .collect();
    }

    pub fn spawnable_positions(&self, color: Color) -> Vec<Position> {
        return self
            .negative_space()
            .iter()
            .filter(|pos| self.spawnable(color, pos))
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
        visited
    }

    pub fn pinned(&self, position: &Position) -> bool {
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
            Some(start) => visited = self.walk_board(start, position, visited.clone()),
            None => return false,
        }
        visited.len() < (len - 1)
    }

    pub fn top_layer_neighbors(&self, position: &Position) -> Vec<Piece> {
        return self
            .positions_around(position)
            .iter()
            .filter_map(|pos| self.board.get(pos).and_then(|v| v.last()))
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

    /// Checks whether a piece of color can be spawned at position
    pub fn spawnable(&self, color: Color, position: &Position) -> bool {
        if self.board.contains_key(position) {
            return false;
        }
        if self.board.keys().len() < 2 {
            return true;
        }
        !self
            .top_layer_neighbors(position)
            .iter()
            .any(|piece| color == piece.color.opposite())
    }

    // TODO:either make this -> Result or make sure that
    pub fn spawn(&mut self, position: &Position, piece: Piece) {
        // TODO: if !self.spawnable(piece.color, position) {
        // TODO:     return Err(format!("Position: {} is not spawnable for {}", position, piece.color);
        // TODO: }
        self.board
            .entry(*position)
            .and_modify(|v| v.push(piece))
            .or_insert_with(|| vec![piece]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_positions_around() {
        let board = Board::new();
        let positions_0_0 = board
            .positions_around(&Position(0, 0))
            .into_iter()
            .collect::<HashSet<Position>>();
        for pos in positions_0_0.iter() {
            let other = board
                .positions_around(pos)
                .into_iter()
                .collect::<HashSet<Position>>();
            assert_eq!(positions_0_0.intersection(&other).count(), 2);
        }
    }

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
        board.spawn(&Position(1, 0), *pieces.last().unwrap());
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        pieces.push(Piece::new(Bug::Beetle, Color::Black, 1));
        board.spawn(&Position(1, 0), *pieces.last().unwrap());
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        board.spawn(&Position(0, 2), Piece::new(Bug::Ladybug, Color::Black, 1));
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces]);
    }

    #[test]
    fn tests_top_layer_neighbors() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, 3));
        board.spawn(&Position(4, 0), Piece::new(Bug::Ant, Color::Black, 3));
        board.spawn(&Position(3, 1), Piece::new(Bug::Ant, Color::Black, 3));
        assert_eq!(board.top_layer_neighbors(&Position(0, 0)).len(), 1);
        assert_eq!(board.top_layer_neighbors(&Position(1, 0)).len(), 2);
        assert_eq!(board.top_layer_neighbors(&Position(2, 0)).len(), 2);
        assert_eq!(board.top_layer_neighbors(&Position(3, 0)).len(), 3);
    }

    #[test]
    fn tests_negative_space() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        let mut positions = board.positions_around(&Position(0, 0));
        let mut negative_space = board.negative_space();
        assert_eq!(negative_space.sort(), positions.sort());
        board.spawn(&Position(0, 1), Piece::new(Bug::Queen, Color::Black, 1));
        assert_eq!(board.negative_space().len(), 8);
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
            if pos == &Position(1, 0) {
                assert_eq!(visited.len(), 2);
            } else {
                assert_eq!(visited.len(), 8);
            }
        }
    }

    #[test]
    fn tests_spawnable_positions() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        let positions = board.spawnable_positions(Color::Black);
        assert_eq!(positions.len(), 3);
        let positions = board.spawnable_positions(Color::White);
        assert_eq!(positions.len(), 3);
        board.spawn(&Position(2, 0), Piece::new(Bug::Ant, Color::White, 2));
        let positions = board.spawnable_positions(Color::White);
        assert_eq!(positions.len(), 6);
        let positions = board.spawnable_positions(Color::Black);
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn tests_spawnable() {
        let mut board = Board::new();
        // if board is empty you can spawn
        assert!(board.spawnable(Color::White, &Position(0, 0)));
        board.spawn(&Position(0, 0), Piece::new(Bug::Ant, Color::White, 1));

        // if position is already occupied, a bug can't be spawned there
        assert!(!board.spawnable(Color::White, &Position(0, 0)));

        // the second bug can always be played
        assert!(board.spawnable(Color::Black, &Position(1, 0)));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));

        // now no other black bug can be spawned around the white one
        for pos in board.positions_around(&Position(0, 0)).iter() {
            assert!(!board.spawnable(Color::Black, pos));
        }

        // a white bug can be added adjacent to a white, but not a black bug
        assert!(!board.spawnable(Color::White, &Position(2, 0)));
        assert!(board.spawnable(Color::White, &Position(-1, 0)));
        assert!(board.spawnable(Color::Black, &Position(2, 0)));
        assert!(!board.spawnable(Color::Black, &Position(-1, 0)));
    }

    #[test]
    fn tests_move_splits_hive() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::Black, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, 3));
        assert!(!board.pinned(&Position(0, 0)));
        assert!(board.pinned(&Position(1, 0)));
        assert!(board.pinned(&Position(2, 0)));
        assert!(!board.pinned(&Position(3, 0)));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Ant, Color::Black, 5));
        }
        for pos in board.positions_around(&Position(0, 0)).iter() {
            if pos == &Position(1, 0) {
                assert!(board.pinned(pos));
            } else {
                assert!(!board.pinned(pos));
            };
        }
    }
}
