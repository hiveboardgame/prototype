use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use crate::bug::Bug;
use crate::color::Color;
use crate::direction::Direction;
use crate::game_error::GameError;
use crate::game_result::GameResult;
use crate::game_type::GameType;
use crate::piece::Piece;
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
        let ((min_x, min_y), (max_x, max_y)) = self.min_max_positions();
        let mut s = "".to_string();
        for y in min_y..=max_y {
            if y.rem_euclid(2) == 1 {
                write!(s, "  ")?;
            }
            for x in min_x..=max_x {
                match self.board.get(&Position(x, y)) {
                    Some(piece) => match piece.last() {
                        Some(last) => write!(s, "{} ", last),
                        None => unreachable!("Found a piece key but no value"),
                    },
                    None => write!(s, "    "),
                }?
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

    pub fn new_from_entomology(input: &str) -> Result<Self, GameError> {
        let mut board = Self {
            board: HashMap::new(),
            last_moved: None,
        };

        let re_ento = Regex::new(r"(([wb][ABGSLPMQ][1-3]?){1,7})([+-]\d)([+-]\d)").unwrap();
        let re_pieces = Regex::new(r"([wb][ABGSLPMQ][1-3]?)").unwrap();
        for ento_cap in re_ento.captures_iter(input) {
            let mut pieces = Vec::new();
            for piece_cap in re_pieces.captures_iter(&ento_cap[1]) {
                let piece = Piece::from_string(&piece_cap[1])?;
                pieces.push(piece);
            }
            pieces.reverse();

            let mut x =
                ento_cap[3]
                    .to_string()
                    .parse::<i8>()
                    .map_err(|_| GameError::ParsingError {
                        found: ento_cap[3].to_string(),
                        typ: "Position(x,_)".to_string(),
                    })?;
            let y = ento_cap[4]
                .to_string()
                .parse::<i8>()
                .map_err(|_| GameError::ParsingError {
                    found: ento_cap[4].to_string(),
                    typ: "Position(_,y)".to_string(),
                })?;
            x = x + (y - 1).div_ceil(2);
            let position = Position(x, y);
            board.board.insert(position, pieces);
        }
        println!("{}", board);
        Ok(board)
    }

    pub fn min_max_positions(&self) -> ((i8, i8), (i8, i8)) {
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

    pub fn game_result(&self) -> GameResult {
        let black = self
            .position_of_piece(&Piece::new(Bug::Queen, Color::White, None))
            .map(|pos| self.neighbors(&pos).len() == 6);
        let white = self
            .position_of_piece(&Piece::new(Bug::Queen, Color::Black, None))
            .map(|pos| self.neighbors(&pos).len() == 6);
        return match (black, white) {
            (Some(true), Some(true)) => GameResult::Draw,
            (Some(true), Some(false)) => GameResult::Winner(Color::Black),
            (Some(false), Some(true)) => GameResult::Winner(Color::White),
            _ => GameResult::Unknown,
        };
    }

    pub fn position_of_piece(&self, piece: &Piece) -> Option<Position> {
        for (pos, pieces) in self.board.iter() {
            if pieces.contains(piece) {
                return Some(*pos);
            }
        }
        None
    }

    pub fn move_piece(
        &mut self,
        piece: &Piece,
        current: &Position,
        target: &Position,
        turn: usize,
    ) -> Result<(), GameError> {
        if !self.is_top_piece(piece, current) {
            return Err(GameError::InvalidMove {
                piece: piece.to_string(),
                from: current.to_string(),
                to: target.to_string(),
                turn,
                reason: "Trying to move a covered piece".to_string(),
            });
        }
        if let Some(vec) = self.board.get_mut(current) {
            if let Some(piece) = vec.pop() {
                if vec.is_empty() {
                    self.board.remove(current);
                }
                self.insert(target, piece);
                return Ok(());
            }
        }
        panic!(
            "Trying to move {} from {} to {} which should have been a legal move",
            piece.to_string(),
            current.to_string(),
            target.to_string()
        );
    }

    pub fn neighbor_is_a(&self, position: &Position, bug: Bug) -> bool {
        self.top_layer_neighbors(position)
            .iter()
            .any(|piece| piece.bug == bug)
    }

    pub fn position(&self, piece: &Piece) -> Option<Position> {
        self.board.iter().find_map(|(pos, pieces)| {
            if pieces.contains(piece) {
                Some(*pos)
            } else {
                None
            }
        })
    }

    pub fn piece_already_played(&self, piece: &Piece) -> bool {
        self.board.values().any(|pieces| pieces.contains(piece))
    }

    pub fn positions_for_color(&self, color: &Color) -> Vec<Position> {
        self.board
            .iter()
            .filter_map(|(pos, pieces)| {
                if let Some(piece) = pieces.last() {
                    if piece.is_color(color) {
                        return Some(*pos);
                    }
                }
                None
            })
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

    pub fn top_piece(&self, position: &Position) -> Option<Piece> {
        if let Some(piece) = self.board.get(position).unwrap_or(&Vec::new()).last() {
            return Some(*piece);
        }
        None
    }

    pub fn is_top_piece(&self, piece: &Piece, position: &Position) -> bool {
        if let Some(found) = self.top_piece(position) {
            return piece == &found;
        }
        false
    }

    pub fn top_bug(&self, position: &Position) -> Option<Bug> {
        if let Some(piece) = self.top_piece(position) {
            return Some(piece.bug);
        }
        None
    }

    pub fn gated(&self, level: usize, from: &Position, to: &Position) -> bool {
        let (pos1, pos2) = from.common_adjacent_positions(to);
        match (self.board.get(&pos1), self.board.get(&pos2)) {
            (Some(p1), Some(p2)) => p1.len() >= level && p2.len() >= level,
            _ => false,
        }
    }

    pub fn get_neighbor(&self, position: &Position) -> Option<(Piece, Position)> {
        for pos in self.positions_around(position).iter() {
            if let Some(pieces) = self.board.get(pos) {
                if let Some(piece) = pieces.last() {
                    return Some((*piece, *pos));
                }
            }
        }
        None
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
            // because the only unkown input here is it is okay to use top_piece_trusted here
            // color, everthing else in known to be correct,
            if let Some(piece) = self.top_piece(pos) {
                // that are the correct color
                if piece.is_color(color) {
                    // let's make sure pieces that were just moved cannot be moved again
                    if let Some(last_moved) = self.last_moved {
                        if last_moved == (piece, *pos) {
                            // now we skip it
                            continue;
                        }
                    }
                    // get all the moves
                    for (start_pos, target_positions) in Bug::available_moves(pos, self) {
                        if let Some(piece) = self.top_piece(&start_pos) {
                            moves
                                .entry((piece, start_pos))
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

    pub fn spawns_left(&self, color: &Color, game_type: GameType) -> bool {
        let reserve_bugs_count = self
            .reserve(color, game_type)
            .iter()
            .fold(0, |acc, (_bug, count)| acc + count);
        self.spawnable_positions(color).len() > 0 && reserve_bugs_count > 0
    }

    pub fn reserve(&self, color: &Color, game_type: GameType) -> HashMap<Bug, i8> {
        let mut bugs = Bug::bugs_count(game_type);
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
        board.insert(
            &Position(1, 0),
            *pieces.last().expect("This is in test neighbors"),
        );
        let neighbors = board.neighbors(&Position(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        pieces.push(Piece::new(Bug::Beetle, Color::Black, Some(1)));
        board.insert(
            &Position(1, 0),
            *pieces.last().expect("This is in test neighbors"),
        );
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

    #[test]
    fn tests_new_from_entomology() {
        assert!(Board::new_from_entomology(
            "bL+0+0wL+1-1bB1wQ+2-1bA2+3-1bM+3-2wMbQ-1+0wA3-1-1wA1-2+0wA2-2+1wB1-3+1bA1-4+1"
        )
        .is_ok());
        assert!(Board::new_from_entomology("wL+0+0wP+0-1bP+1+0bA1+1+1wM+1-1wA1+2+1wB1bQ+2-1wA3+3-1wA2+3-2bB1-1+1wQ-1-1bA3-1-2bA2-2+0bG1-2-1").is_ok());
        assert!(Board::new_from_entomology("bG3+0+0wQ+0-2bA2+0-3bS1+1+0bG1+1-3bB2+2-1wP+2-3wA1+3-1bS2+4-1wA2+5-1bG2-1+1bQ-1+2bA3-1+3wL-1+4bA1-1-1wS1-1-3wB2bB1-2+0wB1bP-2+1wA3-2+3wMbM-3+2bL-3+3").is_ok());
        assert!(Board::new_from_entomology("wG1bG2+0+0wQ+0+1bB2+0-2wB2+1+1bB1+1-1bS1+1-2wA1+2+1bS2+2-1bG2+2-3bG3+2-4wA2+3+1bQ+3-1bA3+3-5bG1+4-1wG2-1+0wS1-1+2wS2-1+3bA1-1-2wG3-2+0wA3-2+4bA2-2-2wB1-3+0").is_ok());
    }
}
