use fnv::FnvHashMap;
type HashMap<K, V> = FnvHashMap<K, V>;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{self, Write};

use crate::bug::Bug;
use crate::color::Color;
use crate::direction::Direction;
use crate::game_error::GameError;
use crate::game_result::GameResult;
use crate::game_type::GameType;
use crate::piece::Piece;
use crate::position::Position;
use crate::torus_array::TorusArray;

pub static BOARD_SIZE: i32 = 32;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub board: TorusArray<Vec<Piece>>,
    pub last_moved: Option<(Piece, Position)>,
    pub piece_positions: Vec<Option<Position>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: TorusArray::new(vec![]),
            last_moved: None,
            piece_positions: vec![None; 48],
        }
    }

    pub fn game_result(&self) -> GameResult {
        let black = self
            .position_of_piece(Piece::new(Bug::Queen, Color::White, None))
            .map(|pos| self.neighbors(pos).len() == 6);
        let white = self
            .position_of_piece(Piece::new(Bug::Queen, Color::Black, None))
            .map(|pos| self.neighbors(pos).len() == 6);
        match (black, white) {
            (Some(true), Some(true)) => GameResult::Draw,
            (Some(true), Some(false)) => GameResult::Winner(Color::Black),
            (Some(false), Some(true)) => GameResult::Winner(Color::White),
            _ => GameResult::Unknown,
        }
    }

    pub fn set_position_of_piece(&mut self, piece: Piece, position: Position) {
        let number_of_bugs = 24;
        let color = piece.color as usize;
        let bug = piece.bug as usize;
        let num = piece.order.unwrap_or(1) as usize;
        self.piece_positions[color * number_of_bugs + bug * 3 + num] = Some(position);
    }

    pub fn position_of_piece(&self, piece: Piece) -> Option<Position> {
        let number_of_bugs = 24;
        let color = piece.color as usize;
        let bug = piece.bug as usize;
        let num = piece.order.unwrap_or(1) as usize;
        self.piece_positions
            .get(color * number_of_bugs + bug * 3 + num)
            .expect("The vec gets initialized to have space for all the bugs")
            .clone()
    }

    pub fn piece_already_played(&self, piece: Piece) -> bool {
        self.position_of_piece(piece).is_some()
    }

    pub fn move_piece(
        &mut self,
        piece: Piece,
        current: Position,
        target: Position,
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
        let vec = self.board.get_mut(current);
        if let Some(piece) = vec.pop() {
            self.insert(target, piece);
            return Ok(());
        }
        unreachable!(
            "Trying to move {piece} from {current} to {target} which should have been a legal move"
        );
    }

    pub fn neighbor_is_a(&self, position: Position, bug: Bug) -> bool {
        self.top_layer_neighbors(position)
            .iter()
            .any(|piece| piece.bug == bug)
    }

    pub fn positions_around_iter(
        &self,
        position: Position,
    ) -> impl Iterator<Item = Position> + 'static {
        static DIRS: [Direction; 6] = [
            Direction::NW,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
        ];
        DIRS.iter().map(move |dir| position.to(dir))
    }

    pub fn positions_around(&self, position: Position) -> Vec<Position> {
        vec![
            position.to(&Direction::NW),
            position.to(&Direction::NE),
            position.to(&Direction::E),
            position.to(&Direction::SE),
            position.to(&Direction::SW),
            position.to(&Direction::W),
        ]
    }

    pub fn level(&self, position: Position) -> usize {
        self.board.get(position).len()
    }

    pub fn top_piece(&self, position: Position) -> Option<Piece> {
        self.board.get(position).last().cloned()
    }

    pub fn is_top_piece(&self, piece: Piece, position: Position) -> bool {
        self.top_piece(position)
            .map(|found| found == piece)
            .unwrap_or(false)
    }

    pub fn top_bug(&self, position: Position) -> Option<Bug> {
        if let Some(piece) = self.top_piece(position) {
            return Some(piece.bug);
        }
        None
    }

    pub fn gated(&self, level: usize, from: Position, to: Position) -> bool {
        let (pos1, pos2) = from.common_adjacent_positions(to);
        let p1 = self.board.get(pos1);
        let p2 = self.board.get(pos2);
        if p1.len() == 0 || p2.len() == 0 {
            return false;
        }
        p1.len() >= level && p2.len() >= level
    }

    pub fn get_neighbor(&self, position: Position) -> Option<(Piece, Position)> {
        for pos in self.positions_around(position).into_iter() {
            let pieces = self.board.get(pos);
            if let Some(piece) = pieces.last() {
                return Some((*piece, pos));
            }
        }
        None
    }

    pub fn positions_taken_around_iter<'this>(
        &'this self,
        position: Position,
    ) -> impl Iterator<Item = Position> + 'this {
        self.positions_around_iter(position)
            .filter(|pos| self.occupied(*pos))
    }

    pub fn occupied(&self, position: Position) -> bool {
        self.board.get(position).len() > 0
    }

    pub fn positions_taken_around(&self, position: Position) -> Vec<Position> {
        self.positions_around(position)
            .into_iter()
            .filter(|pos| self.occupied(*pos))
            .collect()
    }

    pub fn positions_available_around(&self, position: Position) -> Vec<Position> {
        self.positions_around(position)
            .into_iter()
            .filter(|pos| !self.occupied(*pos))
            .collect()
    }

    pub fn neighbors(&self, position: Position) -> Vec<Vec<Piece>> {
        return self
            .positions_around(position)
            .iter()
            .filter_map(|pos| {
                if self.occupied(*pos) {
                    Some(self.board.get(*pos))
                } else {
                    None
                }
            })
            .cloned()
            .collect();
    }

    pub fn is_valid_move(
        &self,
        color: Color,
        piece: Piece,
        current_position: Position,
        target_position: Position,
    ) -> bool {
        return match self.moves(color).get(&(piece, current_position)) {
            None => false,
            Some(positions) => positions.contains(&target_position),
        };
    }

    pub fn moves(&self, color: Color) -> HashMap<(Piece, Position), Vec<Position>> {
        // TODO this needs caching
        let mut moves: HashMap<(Piece, Position), Vec<Position>> = HashMap::default();
        if !self.queen_played(color) {
            return moves;
        }
        for x in 0..32 {
            for y in 0..32 {
                let pos = Position { x, y };
                if let Some(piece) = self.top_piece(pos) {
                    if piece.is_color(color) {
                        // let's make sure pieces that were just moved cannot be moved again
                        if let Some(last_moved) = self.last_moved {
                            if last_moved == (piece, pos) {
                                // now we skip it
                                continue;
                            }
                        }
                        for (start_pos, target_positions) in Bug::available_moves(pos, self) {
                            if let Some(piece) = self.top_piece(start_pos) {
                                moves
                                    .entry((piece, start_pos))
                                    .or_default()
                                    .append(&mut target_positions.clone());
                            }
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

    pub fn spawnable_positions(&self, color: Color) -> Vec<Position> {
        if !self.piece_positions.iter().any(|piece| piece.is_some()) {
            return vec![Position::inital_spawn_position()];
        }
        self.negative_space()
            .iter()
            .filter(|pos| self.spawnable(color, **pos))
            .cloned()
            .collect()
    }

    pub fn queen_played(&self, color: Color) -> bool {
        self.piece_already_played(Piece::new(Bug::Queen, color, None))
    }

    pub fn queen_required(&self, turn: usize, color: Color) -> bool {
        if turn == 6 && color == Color::White && !self.queen_played(Color::White) {
            return true;
        }
        if turn == 7 && color == Color::Black && !self.queen_played(Color::Black) {
            return true;
        }
        false
    }

    fn walk_board(
        &self,
        position: Position,
        excluded_position: Position,
        mut visited: HashSet<Position>,
    ) -> HashSet<Position> {
        self.walk_board_inner(position, excluded_position, &mut visited);
        visited
    }

    fn walk_board_inner(
        &self,
        position: Position,
        excluded_position: Position,
        mut visited: &mut HashSet<Position>,
    ) {
        if visited.contains(&position) {
            return;
        }
        visited.insert(position);
        for pos in self.positions_taken_around_iter(position) {
            if pos != excluded_position && !visited.contains(&pos) {
                self.walk_board_inner(pos, excluded_position, &mut visited);
            }
        }
    }

    pub fn pinned(&self, position: Position) -> bool {
        // pieces on top of the hive cannot be pinned (just gated)
        if self.level(position) > 1 {
            return false;
        }
        // if there's only one neighbor the piece isn't pinned
        let all_neighbor_positions = self.positions_taken_around(position);
        if all_neighbor_positions.len() < 2 {
            return false;
        }
        let mut visited = HashSet::new();
        visited = self.walk_board(*all_neighbor_positions.last().unwrap(), position, visited);
        for neighbor_pos in all_neighbor_positions {
            // if we can't reach all neighbors starting from a random neighbor, the piece would
            // break the hive into two
            if !visited.contains(&neighbor_pos) {
                return true;
            }
        }
        false
    }

    pub fn top_layer_neighbors(&self, position: Position) -> Vec<Piece> {
        return self
            .positions_around(position)
            .iter()
            .filter_map(|pos| self.board.get(*pos).last())
            .cloned()
            .collect();
    }

    pub fn spawns_left(&self, color: Color, game_type: GameType) -> bool {
        let reserve_bugs_count = self
            .reserve(color, game_type)
            .iter()
            .fold(0, |acc, (_bug, count)| acc + count);
        !self.spawnable_positions(color).is_empty() && reserve_bugs_count > 0
    }

    pub fn reserve(&self, color: Color, game_type: GameType) -> HashMap<Bug, i8> {
        // TODO Cache the shit out of this, too plz
        let mut bugs = Bug::bugs_count(game_type);
        for x in 0..32 {
            for y in 0..32 {
                for piece in self.board.get(Position { x, y }) {
                    if piece.is_color(color) {
                        if let Some(i) = bugs.get_mut(&piece.bug) {
                            *i -= 1;
                        }
                    }
                }
            }
        }
        bugs
    }

    pub fn negative_space(&self) -> Vec<Position> {
        let mut negative_space = HashSet::new();
        for maybe_pos in self.piece_positions.iter() {
            if let Some(pos) = maybe_pos {
                for neighbor in self.positions_around(*pos).into_iter() {
                    if self.is_negative_space(neighbor) {
                        negative_space.insert(neighbor);
                    }
                }
            }
        }
        Vec::from_iter(negative_space)
    }

    pub fn is_negative_space(&self, position: Position) -> bool {
        if self.board.get(position).len() == 0 {
            return self.positions_taken_around_iter(position).count() > 0;
        }
        false
    }

    pub fn all_taken_positions(&self) -> impl Iterator<Item = Position> {
        // TODO caching
        let mut taken = Vec::new();
        for x in 0..32 {
            for y in 0..32 {
                let position = Position { x, y };
                if self.occupied(position) {
                    taken.push(position)
                }
            }
        }
        taken.into_iter()
    }

    pub fn spawnable(&self, color: Color, position: Position) -> bool {
        if self.occupied(position) {
            return false;
        }
        let number_of_positions = self.all_taken_positions().count();
        if number_of_positions == 0 {
            return position == Position::inital_spawn_position();
        }
        if number_of_positions == 1 {
            return self.is_negative_space(position);
        }
        !self
            .top_layer_neighbors(position)
            .iter()
            .any(|piece| color == piece.color.opposite())
    }

    pub fn insert(&mut self, position: Position, piece: Piece) {
        self.last_moved = Some((piece, position));
        self.board.get_mut(position).push(piece);
        self.set_position_of_piece(piece, position);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for y in 0..BOARD_SIZE {
            if y.rem_euclid(2) == 1 {
                write!(s, "  ")?;
            }
            for x in 0..BOARD_SIZE {
                let pieces = self.board.get(Position::new_i32(x, y));
                if let Some(last) = pieces.last() {
                    if last.to_string().len() < 3 {
                        write!(s, "{last}  ")?;
                    } else {
                        write!(s, "{last} ")?;
                    }
                } else {
                    write!(s, "    ")?;
                }
            }
            writeln!(s)?;
        }
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_positions_around() {
        let board = Board::new();
        let positions_0_0 = board
            .positions_around(Position::new(0, 0))
            .into_iter()
            .collect::<HashSet<Position>>();
        for pos in positions_0_0.clone().into_iter() {
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
            Position::new(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new(Bug::Ant, Color::Black, Some(1)),
        );
        let pos = board.positions_taken_around(Position::new(0, 0));
        assert_eq!(pos, vec![Position::new(1, 0)]);
    }

    #[test]
    fn tests_neighbors() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        let mut pieces = vec![Piece::new(Bug::Ant, Color::Black, Some(1))];
        board.insert(
            Position::new(1, 0),
            *pieces.last().expect("This is in test neighbors"),
        );
        let neighbors = board.neighbors(Position::new(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        pieces.push(Piece::new(Bug::Beetle, Color::Black, Some(1)));
        board.insert(
            Position::new(1, 0),
            *pieces.last().expect("This is in test neighbors"),
        );
        let neighbors = board.neighbors(Position::new(0, 0));
        assert_eq!(neighbors, vec![pieces.clone()]);

        board.insert(
            Position::new(0, 2),
            Piece::new(Bug::Ladybug, Color::Black, Some(1)),
        );
        let neighbors = board.neighbors(Position::new(0, 0));
        assert_eq!(neighbors, vec![pieces]);
    }

    #[test]
    fn tests_top_layer_neighbors() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new(Bug::Ant, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(2, 0),
            Piece::new(Bug::Ant, Color::Black, Some(2)),
        );
        board.insert(
            Position::new(3, 0),
            Piece::new(Bug::Ant, Color::Black, Some(3)),
        );
        board.insert(
            Position::new(4, 0),
            Piece::new(Bug::Ant, Color::Black, Some(3)),
        );
        board.insert(
            Position::new(3, 1),
            Piece::new(Bug::Ant, Color::Black, Some(3)),
        );
        assert_eq!(board.top_layer_neighbors(Position::new(0, 0)).len(), 1);
        assert_eq!(board.top_layer_neighbors(Position::new(1, 0)).len(), 2);
        assert_eq!(board.top_layer_neighbors(Position::new(2, 0)).len(), 2);
        assert_eq!(board.top_layer_neighbors(Position::new(3, 0)).len(), 3);
    }

    #[test]
    fn tests_negative_space() {
        let mut board = Board::new();
        board.insert(
            Position::inital_spawn_position(),
            Piece::new(Bug::Queen, Color::White, None),
        );
        let mut positions = board.positions_around(Position::inital_spawn_position());
        let mut negative_space = board.negative_space();
        assert_eq!(negative_space.sort(), positions.sort());
        board.insert(
            Position::inital_spawn_position().to(&Direction::NW),
            Piece::new(Bug::Queen, Color::Black, None),
        );
        assert_eq!(board.negative_space().len(), 8);
    }

    #[test]
    fn tests_walk_board() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new(Bug::Ant, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(2, 0),
            Piece::new(Bug::Ant, Color::Black, Some(2)),
        );
        board.insert(
            Position::new(3, 0),
            Piece::new(Bug::Ant, Color::Black, Some(3)),
        );
        let excluded = Position::new(5, 0);
        let visited = board.walk_board(Position::new(0, 0), excluded, HashSet::new());
        assert_eq!(visited.len(), 4);
        let excluded = Position::new(2, 0);
        let visited = board.walk_board(Position::new(0, 0), excluded, HashSet::new());
        assert_eq!(visited.len(), 2);
        let visited = board.walk_board(Position::new(0, 0), excluded, HashSet::new());
        assert_eq!(visited.len(), 2);
        let visited = board.walk_board(Position::new(1, 0), excluded, HashSet::new());
        assert_eq!(visited.len(), 2);
        let visited = board.walk_board(Position::new(3, 0), excluded, HashSet::new());
        assert_eq!(visited.len(), 1);

        for pos in board.positions_around(Position::new(0, 0)).into_iter() {
            board.insert(pos, Piece::new(Bug::Ant, Color::Black, Some(5)));
        }
        for pos in board.positions_around(Position::new(0, 0)).into_iter() {
            let visited = board.walk_board(Position::new(3, 0), pos, HashSet::new());
            if pos == Position::new(1, 0) {
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
            Position::inital_spawn_position(),
            Piece::new(Bug::Queen, Color::White, Some(1)),
        );
        board.insert(
            Position::inital_spawn_position().to(&Direction::E),
            Piece::new(Bug::Ant, Color::Black, Some(1)),
        );
        let positions = board.spawnable_positions(Color::Black);
        assert_eq!(positions.len(), 3);
        let positions = board.spawnable_positions(Color::White);
        assert_eq!(positions.len(), 3);
        board.insert(
            Position::inital_spawn_position()
                .to(&Direction::E)
                .to(&Direction::E),
            Piece::new(Bug::Ant, Color::White, Some(2)),
        );
        let positions = board.spawnable_positions(Color::White);
        assert_eq!(positions.len(), 6);
        let positions = board.spawnable_positions(Color::Black);
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn tests_spawnable() {
        let mut board = Board::new();
        // if board is empty you can spawn
        assert!(board.spawnable(Color::White, Position::inital_spawn_position()));
        board.insert(
            Position::inital_spawn_position(),
            Piece::new(Bug::Ant, Color::White, Some(1)),
        );

        // if position is already occupied, a bug can't be spawned there
        assert!(!board.spawnable(Color::White, Position::inital_spawn_position()));

        // the second bug can always be played
        assert!(board.spawnable(
            Color::Black,
            Position::inital_spawn_position().to(&Direction::E)
        ));
        board.insert(
            Position::inital_spawn_position().to(&Direction::E),
            Piece::new(Bug::Ant, Color::Black, Some(1)),
        );

        // now no other black bug can be spawned around the white one
        for pos in board
            .positions_around(Position::inital_spawn_position())
            .iter()
        {
            assert!(!board.spawnable(Color::Black, *pos));
        }

        // a white bug can be added adjacent to a white, but not a black bug
        assert!(!board.spawnable(
            Color::White,
            Position::inital_spawn_position()
                .to(&Direction::E)
                .to(&Direction::E)
        ));
        assert!(board.spawnable(
            Color::White,
            Position::inital_spawn_position().to(&Direction::W)
        ));
        assert!(board.spawnable(
            Color::Black,
            Position::inital_spawn_position()
                .to(&Direction::E)
                .to(&Direction::E)
        ));
        assert!(!board.spawnable(
            Color::Black,
            Position::inital_spawn_position().to(&Direction::W)
        ));
    }

    #[test]
    fn tests_move_splits_hive() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new(Bug::Queen, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new(Bug::Ant, Color::Black, Some(1)),
        );
        board.insert(
            Position::new(2, 0),
            Piece::new(Bug::Ant, Color::Black, Some(2)),
        );
        board.insert(
            Position::new(3, 0),
            Piece::new(Bug::Ant, Color::Black, Some(3)),
        );
        assert!(!board.pinned(Position::new(0, 0)));
        assert!(board.pinned(Position::new(1, 0)));
        assert!(board.pinned(Position::new(2, 0)));
        assert!(!board.pinned(Position::new(3, 0)));
        for pos in board.positions_around(Position::new(0, 0)).iter() {
            if pos == &Position::new(1, 0) {
                continue;
            }
            board.insert(*pos, Piece::new(Bug::Ant, Color::Black, Some(5)));
        }
        for pos in board.positions_around(Position::new(0, 0)).iter() {
            if pos == &Position::new(1, 0) {
                assert!(board.pinned(*pos));
            } else {
                assert!(!board.pinned(*pos));
            };
        }
    }
}
