use fnv::FnvHashMap;
type HashMap<K, V> = FnvHashMap<K, V>;

use std::collections::HashSet;
use std::fmt::{self, Write};

use crate::{
    bug::Bug, bug_stack::BugStack, color::Color, game_error::GameError, game_result::GameResult,
    game_type::GameType, piece::Piece, position::Position, torus_array::TorusArray,
};

pub const BOARD_SIZE: i32 = 32;

#[derive(Clone, Debug)]
pub struct PinnedInfo {
    pub position: Position,
    pub parent: Option<usize>,
    pub piece: Piece,
    pub visited: bool,
    pub depth: usize,
    pub low: usize,
    pub pinned: bool,
}

impl fmt::Display for PinnedInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {}", self.piece, self.pinned, self.visited)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub board: TorusArray<BugStack>,
    pub last_moved: Option<(Piece, Position)>,
    pub positions: [Option<Position>; 48],
    pub pinned: [bool; 48],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: TorusArray::new(BugStack::new()),
            last_moved: None,
            positions: [None; 48],
            pinned: [false; 48],
        }
    }

    pub fn game_result(&self) -> GameResult {
        let black = self
            .position_of_piece(Piece::new_from(Bug::Queen, Color::White, 0))
            .map(|pos| self.neighbors(pos).len() == 6);
        let white = self
            .position_of_piece(Piece::new_from(Bug::Queen, Color::Black, 0))
            .map(|pos| self.neighbors(pos).len() == 6);
        match (black, white) {
            (Some(true), Some(true)) => GameResult::Draw,
            (Some(true), Some(false)) => GameResult::Winner(Color::Black),
            (Some(false), Some(true)) => GameResult::Winner(Color::White),
            _ => GameResult::Unknown,
        }
    }

    pub fn set_position_of_piece(&mut self, piece: Piece, position: Position) {
        self.positions[self.piece_to_offset(piece)] = Some(position);
    }

    pub fn position_of_piece(&self, piece: Piece) -> Option<Position> {
        *self
            .positions
            .get(self.piece_to_offset(piece))
            .expect("The vec gets initialized to have space for all the bugs")
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
        let bug_stack = self.board.get_mut(current);
        let piece = bug_stack.pop_piece();
        self.insert(target, piece);
        Ok(())
    }

    pub fn neighbor_is_a(&self, position: Position, bug: Bug) -> bool {
        self.top_layer_neighbors(position)
            .iter()
            .any(|piece| piece.bug() == bug)
    }

    pub fn level(&self, position: Position) -> usize {
        self.board.get(position).size as usize
    }

    pub fn piece_to_offset(&self, piece: Piece) -> usize {
        piece.color() as usize * 24 + piece.bug() as usize * 3 + piece.order().saturating_sub(1)
    }

    pub fn offset_to_piece(&self, offset: usize) -> Piece {
        let color = offset as u8 / 24;
        let bug = (offset as u8 - color * 24) / 3;
        let order = (offset as u8 + 1 - bug * 3 - color * 24) as usize;
        Piece::new_from(Bug::from(bug), Color::from(color), order)
    }

    pub fn is_pinned(&self, piece: Piece) -> bool {
        let position = self
            .position_of_piece(piece)
            .expect("Piece not found on board");
        self.pinned[self.piece_to_offset(piece)] && self.board.get(position).len() == 1
    }

    pub fn top_piece(&self, position: Position) -> Option<Piece> {
        self.board.get(position).top_piece()
    }

    pub fn is_top_piece(&self, piece: Piece, position: Position) -> bool {
        self.top_piece(position)
            .map(|found| found == piece)
            .unwrap_or(false)
    }

    pub fn top_bug(&self, position: Position) -> Option<Bug> {
        if let Some(piece) = self.top_piece(position) {
            return Some(piece.bug());
        }
        None
    }

    pub fn gated(&self, level: usize, from: Position, to: Position) -> bool {
        let (pos1, pos2) = from.common_adjacent_positions(to);
        let p1 = self.board.get(pos1);
        let p2 = self.board.get(pos2);
        if p1.is_empty() || p2.is_empty() {
            return false;
        }
        p1.len() >= level && p2.len() >= level
    }

    pub fn get_neighbor(&self, position: Position) -> Option<(Piece, Position)> {
        for pos in position.positions_around() {
            let pieces = self.board.get(pos);
            if let Some(piece) = pieces.top_piece() {
                return Some((piece, pos));
            }
        }
        None
    }

    pub fn positions_taken_around_iter(
        &self,
        position: Position,
    ) -> impl Iterator<Item = Position> + '_ {
        position
            .positions_around()
            .filter(|pos| self.occupied(*pos))
    }

    pub fn occupied(&self, position: Position) -> bool {
        self.board.get(position).size > 0
    }

    pub fn positions_available_around(&self, position: Position) -> Vec<Position> {
        position
            .positions_around()
            .filter(|pos| !self.occupied(*pos))
            .collect()
    }

    pub fn neighbors(&self, position: Position) -> Vec<BugStack> {
        position
            .positions_around()
            .filter_map(|pos| {
                if self.occupied(pos) {
                    Some(self.board.get(pos))
                } else {
                    None
                }
            })
            .cloned()
            .collect()
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
        let mut moves: HashMap<(Piece, Position), Vec<Position>> = HashMap::default();
        if !self.queen_played(color) {
            return moves;
        }
        for pos in self.positions.iter().flatten() {
            if let Some(piece) = self.top_piece(*pos) {
                if piece.is_color(color) {
                    // let's make sure pieces that were just moved cannot be moved again
                    if let Some(last_moved) = self.last_moved {
                        if last_moved == (piece, *pos) {
                            // now we skip it
                            continue;
                        }
                    }
                    for (start_pos, target_positions) in Bug::available_moves(*pos, self) {
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

        if let Some(last_moved) = self.last_moved {
            moves.remove(&last_moved);
        }
        moves
    }

    pub fn spawnable_positions(&self, color: Color) -> Vec<Position> {
        if !self.positions.iter().any(|position| position.is_some()) {
            return vec![Position::inital_spawn_position()];
        }
        self.negative_space()
            .iter()
            .filter(|pos| self.spawnable(color, **pos))
            .cloned()
            .collect()
    }

    pub fn queen_played(&self, color: Color) -> bool {
        self.piece_already_played(Piece::new_from(Bug::Queen, color, 0))
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

    pub fn update_pinned(&mut self) {
        for pinned_info in self.calculate_pinned().iter() {
            self.pinned[self.piece_to_offset(pinned_info.piece)] = pinned_info.pinned
        }
    }

    pub fn calculate_pinned(&self) -> Vec<PinnedInfo> {
        // make sure to get only top pieces in this
        let mut ap_info = self
            .positions
            .iter()
            .enumerate()
            .filter_map(|(i, maybe_pos)| {
                if let Some(pos) = maybe_pos {
                    if self.is_top_piece(self.offset_to_piece(i), *pos) {
                        Some(PinnedInfo {
                            position: *pos,
                            piece: self.top_piece(*pos).unwrap(),
                            visited: false,
                            depth: 0,
                            low: 0,
                            pinned: false,
                            parent: None,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if ap_info.len() == 0 {
            return ap_info;
        }
        self.bcc(0, 0, &mut ap_info);
        ap_info
    }

    pub fn bcc(&self, i: usize, d: usize, ap_info: &mut Vec<PinnedInfo>) {
        ap_info[i].visited = true;
        ap_info[i].depth = d;
        ap_info[i].low = d;
        let mut child_count = 0;
        let mut ap = false;

        for pos in self.positions_taken_around_iter(ap_info[i].position) {
            let ni = ap_info.iter().position(|e| e.position == pos).unwrap();
            if !ap_info[ni].visited {
                child_count += 1;
                ap_info[ni].parent = Some(i);
                self.bcc(ni, d + 1, ap_info);
                if ap_info[ni].low >= ap_info[i].depth {
                    ap = true;
                }
                ap_info[i].low = std::cmp::min(ap_info[i].low, ap_info[ni].low);
            } else {
                if ap_info[i].parent.is_some() && ni != ap_info[i].parent.unwrap() {
                    ap_info[i].low = std::cmp::min(ap_info[i].low, ap_info[ni].depth);
                }
            }
        }
        if ap_info[i].parent.is_some() && ap || (ap_info[i].parent.is_none() && child_count > 1) {
            ap_info[i].pinned = true;
        }
    }

    pub fn top_layer_neighbors(&self, position: Position) -> Vec<Piece> {
        position
            .positions_around()
            .filter_map(|pos| self.board.get(pos).top_piece())
            .collect()
    }

    pub fn spawns_left(&self, color: Color, game_type: GameType) -> bool {
        let reserve_bugs_count = self
            .reserve(color, game_type)
            .iter()
            .fold(0, |acc, (_bug, count)| acc + count);
        !self.spawnable_positions(color).is_empty() && reserve_bugs_count > 0
    }

    pub fn reserve(&self, color: Color, game_type: GameType) -> HashMap<Bug, i8> {
        let start = 24 * color as usize;
        let end = 24 + start;
        let mut bugs = Bug::bugs_count(game_type);
        for (i, maybe_pos) in self.positions[start..end].iter().enumerate() {
            if maybe_pos.is_some() {
                let bug = Bug::from(i as u8 / 3);
                if let Some(num) = bugs.get_mut(&bug) {
                    *num -= 1;
                }
            }
        }
        bugs
    }

    pub fn negative_space(&self) -> Vec<Position> {
        let mut negative_space = HashSet::new();
        for pos in self.positions.iter().flatten() {
            for neighbor in pos.positions_around() {
                if self.is_negative_space(neighbor) {
                    negative_space.insert(neighbor);
                }
            }
        }
        Vec::from_iter(negative_space)
    }

    pub fn is_negative_space(&self, position: Position) -> bool {
        if self.board.get(position).is_empty() {
            return self.positions_taken_around_iter(position).count() > 0;
        }
        false
    }

    pub fn all_taken_positions(&self) -> impl Iterator<Item = Position> {
        // TODO this does not uniq!
        self.positions.into_iter().flatten()
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
            .any(|piece| color == Color::from(piece.color().opposite()))
    }

    pub fn insert(&mut self, position: Position, piece: Piece) {
        self.last_moved = Some((piece, position));
        self.board.get_mut(position).push_piece(piece);
        self.set_position_of_piece(piece, position);
        self.update_pinned();
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
                if let Some(last) = pieces.top_piece() {
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
    use crate::direction::Direction;

    #[test]
    fn tests_positions_around() {
        let positions_0_0 = Position::new(0, 0)
            .positions_around()
            .collect::<HashSet<Position>>();
        for pos in positions_0_0.clone().into_iter() {
            let other = pos.positions_around().collect::<HashSet<Position>>();
            assert_eq!(positions_0_0.intersection(&other).count(), 2);
        }
    }

    #[test]
    fn tests_positions_taken_around() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::Black, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 0),
        );
        let pos = board
            .positions_taken_around_iter(Position::new(0, 0))
            .collect::<Vec<_>>();
        assert_eq!(pos, vec![Position::new(1, 0)]);
    }

    #[test]
    fn tests_neighbors() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::Black, 0),
        );
        let mut bug_stack = BugStack::new();
        let piece = Piece::new_from(Bug::Ant, Color::Black, 1);
        bug_stack.push_piece(piece);
        board.insert(
            Position::new(1, 0),
            bug_stack.top_piece().expect("This is in test neighbors"),
        );
        let neighbors = board.neighbors(Position::new(0, 0));
        assert_eq!(neighbors, vec![bug_stack]);

        bug_stack.push_piece(Piece::new_from(Bug::Beetle, Color::Black, 1));
        board.insert(
            Position::new(1, 0),
            bug_stack.top_piece().expect("This is in test neighbors"),
        );
        let neighbors = board.neighbors(Position::new(0, 0));
        assert_eq!(neighbors, vec![bug_stack]);

        board.insert(
            Position::new(0, 2),
            Piece::new_from(Bug::Ladybug, Color::Black, 0),
        );
        let neighbors = board.neighbors(Position::new(0, 0));
        assert_eq!(neighbors, vec![bug_stack]);
    }

    #[test]
    fn tests_top_layer_neighbors() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::Black, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(2, 0),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(3, 0),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        board.insert(
            Position::new(4, 0),
            Piece::new_from(Bug::Grasshopper, Color::Black, 1),
        );
        board.insert(
            Position::new(3, 1),
            Piece::new_from(Bug::Grasshopper, Color::Black, 2),
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
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        let mut positions = Position::inital_spawn_position()
            .positions_around()
            .collect::<Vec<Position>>();
        let mut negative_space = board.negative_space();
        positions.sort();
        negative_space.sort();
        assert_eq!(negative_space, positions);
        board.insert(
            Position::inital_spawn_position().to(Direction::NW),
            Piece::new_from(Bug::Queen, Color::Black, 0),
        );
        assert_eq!(board.negative_space().len(), 8);
    }

    #[test]
    fn tests_spawnable_positions() {
        let mut board = Board::new();
        board.insert(
            Position::inital_spawn_position(),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::inital_spawn_position().to(Direction::E),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        let positions = board.spawnable_positions(Color::Black);
        assert_eq!(positions.len(), 3);
        let positions = board.spawnable_positions(Color::White);
        assert_eq!(positions.len(), 3);
        board.insert(
            Position::inital_spawn_position()
                .to(Direction::E)
                .to(Direction::E),
            Piece::new_from(Bug::Ant, Color::White, 2),
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
            Piece::new_from(Bug::Ant, Color::White, 1),
        );

        // if position is already occupied, a bug can't be spawned there
        assert!(!board.spawnable(Color::White, Position::inital_spawn_position()));

        // the second bug can always be played
        assert!(board.spawnable(
            Color::Black,
            Position::inital_spawn_position().to(Direction::E)
        ));
        board.insert(
            Position::inital_spawn_position().to(Direction::E),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );

        // now no other black bug can be spawned around the white one
        for pos in Position::inital_spawn_position().positions_around() {
            assert!(!board.spawnable(Color::Black, pos));
        }

        // a white bug can be added adjacent to a white, but not a black bug
        assert!(!board.spawnable(
            Color::White,
            Position::inital_spawn_position()
                .to(Direction::E)
                .to(Direction::E)
        ));
        assert!(board.spawnable(
            Color::White,
            Position::inital_spawn_position().to(Direction::W)
        ));
        assert!(board.spawnable(
            Color::Black,
            Position::inital_spawn_position()
                .to(Direction::E)
                .to(Direction::E)
        ));
        assert!(!board.spawnable(
            Color::Black,
            Position::inital_spawn_position().to(Direction::W)
        ));
    }

    #[test]
    fn tests_move_splits_hive() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::Black, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(2, 0),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(3, 0),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        assert!(!board.pinned(Position::new(0, 0)));
        assert!(board.pinned(Position::new(1, 0)));
        assert!(board.pinned(Position::new(2, 0)));
        assert!(!board.pinned(Position::new(3, 0)));
        for pos in Position::new(0, 0).positions_around() {
            if pos == Position::new(1, 0) {
                continue;
            }
            board.insert(pos, Piece::new_from(Bug::Ant, Color::Black, 1));
        }
        for pos in Position::new(0, 0).positions_around() {
            if pos == Position::new(1, 0) {
                assert!(board.pinned(pos));
            } else {
                assert!(!board.pinned(pos));
            };
        }
    }
}
