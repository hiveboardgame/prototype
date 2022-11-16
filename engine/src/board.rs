use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use crate::bug::Bug;
use crate::color::Color;
use crate::piece::Piece;
use crate::position::Direction;
use crate::position::Position;

#[derive(Deserialize, Serialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct Board {
    pub board: HashMap<Position, Vec<Piece>>,
    pub last_moved: Option<(Piece, Position)>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut positions = self.board.keys().cloned().collect::<Vec<Position>>();
        positions.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        let ((min_x, min_y), (max_x, max_y)) = self.mix_max_positions();
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
    pub fn new() -> Self {
        Self {
            board: HashMap::new(),
            last_moved: None,
        }
    }

    pub fn level_iter<'b>(&'b self) -> impl Iterator<Item = (Position, Piece, usize, bool)> + 'b {
        let len = self
            .board
            .iter()
            .map(|(_, pieces)| pieces.len())
            .max()
            .unwrap_or(0) as usize;
        (0..len).flat_map(|lvl| {
            self.board.iter().filter_map(move |(pos, pieces)| {
                let more = pieces.get(lvl + 1).is_some();
                pieces
                    .get(lvl)
                    .map(|piece| (pos.clone(), piece.clone(), lvl.clone(), more))
            })
        })
    }

    pub fn mix_max_positions(&self) -> ((i8, i8), (i8, i8)) {
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

        return ((min_x, min_y), (max_x, max_y));
    }

    pub fn winner(&self) -> Option<Color> {
        if self
            .position_of_piece(&Piece::new(Bug::Queen, Color::White, None))
            .is_some_and(|pos| self.neighbors(&pos).len() == 6)
        {
            return Some(Color::Black);
        }
        if self
            .position_of_piece(&Piece::new(Bug::Queen, Color::Black, None))
            .is_some_and(|pos| self.neighbors(&pos).len() == 6)
        {
            return Some(Color::White);
        }
        None
    }

    pub fn position_of_piece(&self, piece: &Piece) -> Option<Position> {
        for (pos, pieces) in self.board.iter() {
            if pieces.contains(piece) {
                return Some(*pos);
            }
        }
        None
    }

    pub fn move_piece(&mut self, piece: &Piece, current: &Position, target: &Position) {
        if !self.is_top_piece(piece, current) {
            panic!("Tried to move non top piece!");
        }
        let piece = self.board.get_mut(current).unwrap().pop().unwrap();
        if self.board.get(current).unwrap().is_empty() {
            self.board.remove(current);
        }
        self.insert(target, piece);
    }

    pub fn neighbor_is_a(&self, position: &Position, bug: Bug) -> bool {
        self.top_layer_neighbors(position)
            .iter()
            .any(|piece| piece.bug == bug)
    }

    pub fn position(&self, piece: &Piece) -> Position {
        self.board
            .iter()
            .find_map(|(pos, pieces)| {
                if pieces.contains(piece) {
                    Some(*pos)
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn piece_already_played(&self, piece: &Piece) -> bool {
        self.board.values().any(|pieces| pieces.contains(piece))
    }

    pub fn positions_for_color(&self, color: &Color) -> Vec<Position> {
        self.board
            .keys()
            .filter(|pos| {
                self.board
                    .get(pos)
                    .unwrap() // unwrap here is safe because we are got pos from board.keys
                    .last()
                    .unwrap_or_else(|| panic!("Could not find a piece at pos: {}", pos))
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

    pub fn is_top_piece(&self, piece: &Piece, position: &Position) -> bool {
        self.board.get(position).unwrap().last().unwrap() == piece
    }

    pub fn top_piece(&self, position: &Position) -> Piece {
        *self.board.get(position).unwrap().last().unwrap()
    }

    pub fn top_bug(&self, position: &Position) -> Bug {
        self.board.get(position).unwrap().last().unwrap().bug
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

    pub fn is_valid_move(
        &self,
        color: &Color,
        piece: &Piece,
        current_position: &Position,
        target_position: &Position,
    ) -> bool {
        return match self.moves(color).get(&(*piece, *current_position)) {
            None => false,
            Some(positions) => positions.contains(target_position),
        };
    }

    pub fn moves(&self, color: &Color) -> HashMap<(Piece, Position), Vec<Position>> {
        let mut moves: HashMap<(Piece, Position), Vec<Position>> = HashMap::new();
        // for all pieces on the board
        for pos in self.board.keys() {
            // that are the correct color
            if self.top_piece(pos).is_color(color) {
                // let's make sure pieces that were just moved cannot be moved again
                if let Some(last_moved) = self.last_moved {
                    if last_moved != (self.top_piece(pos), *pos) {
                        // get all the moves
                        for (start_pos, target_positions) in Bug::available_moves(pos, self) {
                            moves
                                .entry((self.top_piece(&start_pos), start_pos))
                                .or_default()
                                .append(&mut target_positions.clone());
                        }
                    }
                }
            }
        }
        if let Some(last_moved) = self.last_moved {
            moves.remove(&last_moved);
        }
        moves
    }

    pub fn spawnable_positions(&self, color: &Color) -> Vec<Position> {
        if self.board.is_empty() {
            return vec![Position(0, 0)];
        }
        return self
            .negative_space()
            .iter()
            .filter(|pos| self.spawnable(color, pos))
            .cloned()
            .collect();
    }

    pub fn queen_played(&self, color: &Color) -> bool {
        return self
            .board
            .values()
            .any(|p| p.contains(&Piece::new(Bug::Queen, *color, None)));
    }

    pub fn queen_required(&self, turn: usize, color: &Color) -> bool {
        if turn == 6 && color == &Color::White && !self.queen_played(&Color::White) {
            return true;
        }
        if turn == 7 && color == &Color::Black && !self.queen_played(&Color::Black) {
            return true;
        }
        return false;
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
        if self.level(position) > 1 {
            return false;
        }
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

    pub fn reserve(&self, color: &Color) -> HashMap<Bug, i8> {
        let mut bugs = Bug::bugs_count();
        for pieces in self.board.values() {
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

    pub fn spawnable(&self, color: &Color, position: &Position) -> bool {
        if self.board.contains_key(position) {
            return false;
        }
        if self.board.keys().len() < 2 {
            return true;
        }
        !self
            .top_layer_neighbors(position)
            .iter()
            .any(|piece| color == &piece.color.opposite())
    }

    pub fn insert(&mut self, position: &Position, piece: Piece) {
        self.last_moved = Some((piece.clone(), position.clone()));
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
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, Some(1)));
        let pos = board.positions_taken_around(&Position(0, 0));
        assert_eq!(pos, vec![Position(1, 0)]);
    }

    #[test]
    fn tests_neighbors() {
        let mut board = Board::new();
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        let mut pieces = vec![Piece::new(Bug::Ant, Color::Black, Some(1))];
        board.insert(&Position(1, 0), *pieces.last().unwrap());
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        pieces.push(Piece::new(Bug::Beetle, Color::Black, Some(1)));
        board.insert(&Position(1, 0), *pieces.last().unwrap());
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        board.insert(
            &Position(0, 2),
            Piece::new(Bug::Ladybug, Color::Black, Some(1)),
        );
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces]);
    }

    #[test]
    fn tests_top_layer_neighbors() {
        let mut board = Board::new();
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, Some(1)));
        board.insert(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, Some(2)));
        board.insert(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, Some(3)));
        board.insert(&Position(4, 0), Piece::new(Bug::Ant, Color::Black, Some(3)));
        board.insert(&Position(3, 1), Piece::new(Bug::Ant, Color::Black, Some(3)));
        assert_eq!(board.top_layer_neighbors(&Position(0, 0)).len(), 1);
        assert_eq!(board.top_layer_neighbors(&Position(1, 0)).len(), 2);
        assert_eq!(board.top_layer_neighbors(&Position(2, 0)).len(), 2);
        assert_eq!(board.top_layer_neighbors(&Position(3, 0)).len(), 3);
    }

    #[test]
    fn tests_negative_space() {
        let mut board = Board::new();
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        let mut positions = board.positions_around(&Position(0, 0));
        let mut negative_space = board.negative_space();
        assert_eq!(negative_space.sort(), positions.sort());
        board.insert(
            &Position(0, 1),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        assert_eq!(board.negative_space().len(), 8);
    }

    #[test]
    fn tests_walk_board() {
        let mut board = Board::new();
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, Some(1)));
        board.insert(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, Some(2)));
        board.insert(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, Some(3)));
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
            board.insert(pos, Piece::new(Bug::Ant, Color::Black, Some(5)));
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
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::White, Some(1)),
        );
        board.insert(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, Some(1)));
        let positions = board.spawnable_positions(&Color::Black);
        assert_eq!(positions.len(), 3);
        let positions = board.spawnable_positions(&Color::White);
        assert_eq!(positions.len(), 3);
        board.insert(&Position(2, 0), Piece::new(Bug::Ant, Color::White, Some(2)));
        let positions = board.spawnable_positions(&Color::White);
        assert_eq!(positions.len(), 6);
        let positions = board.spawnable_positions(&Color::Black);
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn tests_spawnable() {
        let mut board = Board::new();
        // if board is empty you can spawn
        assert!(board.spawnable(&Color::White, &Position(0, 0)));
        board.insert(&Position(0, 0), Piece::new(Bug::Ant, Color::White, Some(1)));

        // if position is already occupied, a bug can't be spawned there
        assert!(!board.spawnable(&Color::White, &Position(0, 0)));

        // the second bug can always be played
        assert!(board.spawnable(&Color::Black, &Position(1, 0)));
        board.insert(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, Some(1)));

        // now no other black bug can be spawned around the white one
        for pos in board.positions_around(&Position(0, 0)).iter() {
            assert!(!board.spawnable(&Color::Black, pos));
        }

        // a white bug can be added adjacent to a white, but not a black bug
        assert!(!board.spawnable(&Color::White, &Position(2, 0)));
        assert!(board.spawnable(&Color::White, &Position(-1, 0)));
        assert!(board.spawnable(&Color::Black, &Position(2, 0)));
        assert!(!board.spawnable(&Color::Black, &Position(-1, 0)));
    }

    #[test]
    fn tests_move_splits_hive() {
        let mut board = Board::new();
        board.insert(
            &Position(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, Some(1)));
        board.insert(&Position(2, 0), Piece::new(Bug::Ant, Color::Black, Some(2)));
        board.insert(&Position(3, 0), Piece::new(Bug::Ant, Color::Black, Some(3)));
        assert!(!board.pinned(&Position(0, 0)));
        assert!(board.pinned(&Position(1, 0)));
        assert!(board.pinned(&Position(2, 0)));
        assert!(!board.pinned(&Position(3, 0)));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            if pos == &Position(1, 0) {
                continue;
            }
            board.insert(pos, Piece::new(Bug::Ant, Color::Black, Some(5)));
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
