use crate::{
    board::Board, direction::Direction, game_error::GameError, game_type::GameType,
    position::Position,
};
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt, str::FromStr};
type HashMap<K, V> = FnvHashMap<K, V>;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum Bug {
    Ant = 0,
    Beetle = 1,
    Grasshopper = 2,
    Ladybug = 3,
    Mosquito = 4,
    Pillbug = 5,
    Queen = 6,
    Spider = 7,
}

impl From<Bug> for u8 {
    fn from(bug: Bug) -> Self {
        bug as u8
    }
}

impl From<u8> for Bug {
    fn from(item: u8) -> Self {
        match item {
            0 => Bug::Ant,
            1 => Bug::Beetle,
            2 => Bug::Grasshopper,
            3 => Bug::Ladybug,
            4 => Bug::Mosquito,
            5 => Bug::Pillbug,
            6 => Bug::Queen,
            7 => Bug::Spider,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Bug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Bug {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Bug::Ant),
            "B" | "b" => Ok(Bug::Beetle),
            "G" | "g" => Ok(Bug::Grasshopper),
            "L" | "l" => Ok(Bug::Ladybug),
            "M" | "m" => Ok(Bug::Mosquito),
            "P" | "p" => Ok(Bug::Pillbug),
            "Q" | "q" => Ok(Bug::Queen),
            "S" | "s" => Ok(Bug::Spider),
            any => Err(GameError::ParsingError {
                found: any.to_string(),
                typ: "bug string".to_string(),
            }),
        }
    }
}

impl Bug {
    pub fn all() -> Vec<Bug> {
        vec![
            Bug::Ant,
            Bug::Beetle,
            Bug::Grasshopper,
            Bug::Ladybug,
            Bug::Mosquito,
            Bug::Pillbug,
            Bug::Queen,
            Bug::Spider,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        &self.name()[0..=0]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Bug::Ant => "Ant",
            Bug::Beetle => "Beetle",
            Bug::Grasshopper => "Grasshopper",
            Bug::Ladybug => "Ladybug",
            Bug::Mosquito => "Mosquito",
            Bug::Pillbug => "Pillbug",
            Bug::Queen => "Queen",
            Bug::Spider => "Spider",
        }
    }

    pub fn as_emoji(&self) -> String {
        match self {
            Bug::Ant => '\u{1f41c}',
            Bug::Beetle => '\u{1fab2}',
            Bug::Grasshopper => '\u{1f997}',
            Bug::Ladybug => '\u{1f41e}',
            Bug::Mosquito => '\u{1f99f}',
            Bug::Pillbug => '\u{1f48a}',
            Bug::Queen => '\u{1f41d}',
            Bug::Spider => '\u{1f577}',
        }
        .clone()
        .to_string()
    }

    pub fn bugs_count(game_type: GameType) -> HashMap<Bug, i8> {
        let mut bugs = HashMap::default();
        bugs.insert(Bug::Ant, 3);
        bugs.insert(Bug::Beetle, 2);
        bugs.insert(Bug::Grasshopper, 3);
        bugs.insert(Bug::Queen, 1);
        bugs.insert(Bug::Spider, 2);
        match game_type {
            GameType::Base => {}
            GameType::M => {
                bugs.insert(Bug::Mosquito, 1);
            }
            GameType::L => {
                bugs.insert(Bug::Ladybug, 1);
            }
            GameType::P => {
                bugs.insert(Bug::Pillbug, 1);
            }
            GameType::ML => {
                bugs.insert(Bug::Mosquito, 1);
                bugs.insert(Bug::Ladybug, 1);
            }
            GameType::MP => {
                bugs.insert(Bug::Mosquito, 1);
                bugs.insert(Bug::Pillbug, 1);
            }
            GameType::LP => {
                bugs.insert(Bug::Ladybug, 1);
                bugs.insert(Bug::Pillbug, 1);
            }
            GameType::MLP => {
                bugs.insert(Bug::Mosquito, 1);
                bugs.insert(Bug::Ladybug, 1);
                bugs.insert(Bug::Pillbug, 1);
            }
        }
        bugs
    }

    pub fn available_moves(position: Position, board: &Board) -> HashMap<Position, Vec<Position>> {
        let mut moves = HashMap::default();
        if !board.pinned(position) {
            let positions = match board.top_bug(position) {
                Some(Bug::Ant) => Bug::ant_moves(position, board),
                Some(Bug::Beetle) => Bug::beetle_moves(position, board),
                Some(Bug::Grasshopper) => Bug::grasshopper_moves(position, board),
                Some(Bug::Ladybug) => Bug::ladybug_moves(position, board),
                Some(Bug::Mosquito) => Bug::mosquito_moves(position, board),
                Some(Bug::Pillbug) => Bug::pillbug_moves(position, board),
                Some(Bug::Queen) => Bug::queen_moves(position, board),
                Some(Bug::Spider) => Bug::spider_moves(position, board),
                None => Vec::new(),
            };
            moves.insert(position, positions);
        }
        moves.extend(Bug::available_abilities(position, board));
        moves
    }

    pub fn available_abilities(
        position: Position,
        board: &Board,
    ) -> HashMap<Position, Vec<Position>> {
        match board.top_bug(position) {
            Some(Bug::Pillbug) => Bug::pillbug_throw(position, board),
            Some(Bug::Mosquito)
                if board.level(position) == 1 && board.neighbor_is_a(position, Bug::Pillbug) =>
            {
                Bug::pillbug_throw(position, board)
            }
            _ => HashMap::default(),
        }
    }

    fn crawl<'board>(
        position: Position,
        board: &'board Board,
    ) -> impl Iterator<Item = Position> + 'board {
        board
            .positions_taken_around_iter(position)
            .flat_map(move |pos| {
                let mut positions = vec![];
                let (pos1, pos2) = position.common_adjacent_positions(pos);
                if !board.gated(1, position, pos1) && !board.occupied(pos1) {
                    positions.push(pos1);
                }
                if !board.gated(1, position, pos2) && !board.occupied(pos2) {
                    positions.push(pos2);
                }
                positions
            })
    }

    fn climb(position: Position, board: &Board) -> Vec<Position> {
        board
            .positions_taken_around(position)
            .iter()
            .filter(|pos| !board.gated(board.level(**pos) + 1, position, **pos))
            .cloned()
            .collect()
    }

    fn descend(position: Position, board: &Board) -> Vec<Position> {
        board
            .positions_available_around(position)
            .iter()
            .filter(|pos| !board.gated(board.level(position), position, **pos))
            .cloned()
            .collect()
    }

    fn ant_moves(position: Position, board: &Board) -> Vec<Position> {
        let mut found = HashSet::new();
        let mut unexplored = HashSet::new();
        unexplored.insert(position);
        // TODO this could be nicer...
        let mut board = board.clone();
        board.board.remove(position);
        Bug::ant_rec(&mut found, &mut unexplored, &board);
        found.remove(&position);
        return found.iter().cloned().collect();
    }

    fn ant_rec(found: &mut HashSet<Position>, unexplored: &mut HashSet<Position>, board: &Board) {
        if let Some(position) = unexplored.iter().next().cloned() {
            unexplored.remove(&position);
            found.insert(position);
            for pos in Bug::crawl(position, board) {
                if !found.contains(&pos) {
                    unexplored.insert(pos);
                }
            }
            Bug::ant_rec(found, unexplored, board)
        }
    }

    pub fn beetle_moves(position: Position, board: &Board) -> Vec<Position> {
        let mut positions = Vec::new();
        for pos in Bug::climb(position, board).into_iter() {
            positions.push(pos);
        }
        if board.level(position) == 1 {
            for pos in Bug::crawl(position, board) {
                if !positions.contains(&pos) {
                    positions.push(pos);
                }
            }
        } else {
            for pos in Bug::descend(position, board).into_iter() {
                if !positions.contains(&pos) {
                    positions.push(pos);
                }
            }
        }
        positions
    }

    pub fn grasshopper_moves(position: Position, board: &Board) -> Vec<Position> {
        // get the directions of the grasshopper's neighbors
        let direction_of_neighbors = board
            .positions_taken_around(position)
            .into_iter()
            .map(|pos| position.direction(pos))
            .collect::<Vec<Direction>>();
        let mut positions = vec![];
        // move in the given direction
        for dir in direction_of_neighbors.iter() {
            let mut cur_pos = position;
            // until there is a free position
            while board.occupied(cur_pos.to(dir)) {
                cur_pos = cur_pos.to(dir);
            }
            // then add the free position
            positions.push(cur_pos.to(dir));
        }
        positions
    }

    fn ladybug_moves(position: Position, board: &Board) -> Vec<Position> {
        // find all adjacent bugs to climb on
        let first = Bug::climb(position, board);
        // stay on top of the hive by performing another climb
        let second: HashSet<Position> = first
            .iter()
            .flat_map(|first_pos| {
                Bug::climb(*first_pos, board)
                    .iter()
                    .filter(|pos| **pos != position && *pos != first_pos)
                    .cloned()
                    .collect::<HashSet<Position>>()
            })
            .collect::<HashSet<Position>>();
        // then find available and ungated positions
        let third: HashSet<Position> = second
            .iter()
            .flat_map(|pos| {
                board
                    .positions_available_around(*pos)
                    .iter()
                    .filter(|p| !board.gated(board.level(*pos) + 1, *pos, **p) && **p != position)
                    .cloned()
                    .collect::<HashSet<Position>>()
            })
            .collect::<HashSet<Position>>();
        return third.iter().cloned().collect();
    }

    fn mosquito_moves(position: Position, board: &Board) -> Vec<Position> {
        return if board.level(position) == 1 {
            board
                .neighbors(position)
                .iter()
                .flat_map(|pieces| {
                    match pieces.top_piece().expect("Could not get last piece").bug() {
                        Bug::Ant => Bug::ant_moves(position, board),
                        Bug::Beetle => Bug::beetle_moves(position, board),
                        Bug::Grasshopper => Bug::grasshopper_moves(position, board),
                        Bug::Ladybug => Bug::ladybug_moves(position, board),
                        Bug::Mosquito => vec![],
                        Bug::Pillbug => Bug::pillbug_moves(position, board),
                        Bug::Queen => Bug::queen_moves(position, board),
                        Bug::Spider => Bug::spider_moves(position, board),
                    }
                })
                .collect()
        } else {
            Bug::beetle_moves(position, board)
        };
    }

    fn pillbug_moves(position: Position, board: &Board) -> Vec<Position> {
        Bug::crawl(position, board).collect()
    }

    fn pillbug_throw(position: Position, board: &Board) -> HashMap<Position, Vec<Position>> {
        let mut moves = HashMap::default();
        // get all the positions the pillbug can throw a bug to
        let to = board
            .positions_available_around(position)
            .iter()
            .filter(|pos| !board.gated(2, position, **pos))
            .cloned()
            .collect::<Vec<Position>>();
        // get bugs around the pillbug that aren't pinned
        for pos in board.positions_taken_around(position).iter().filter(|p| {
            !board.pinned(**p) && !board.gated(2, **p, position) && board.level(**p) <= 1
        }) {
            moves.insert(*pos, to.clone());
        }
        moves
    }

    fn queen_moves(position: Position, board: &Board) -> Vec<Position> {
        Bug::crawl(position, board).collect()
    }

    fn spider_moves(position: Position, board: &Board) -> Vec<Position> {
        let mut moves = vec![vec![position]];
        let mut board = board.clone();
        for i in 0..3 {
            moves = moves
                .iter()
                .flat_map(|positions| {
                    Bug::crawl(*positions.last().expect("Could not get last piece"), &board)
                        .map(|p| {
                            let mut pos = positions.clone();
                            pos.push(p);
                            pos
                        })
                        .collect::<Vec<Vec<Position>>>()
                })
                .collect::<Vec<Vec<Position>>>();
            if i == 0 {
                board.board.remove(position);
            }
        }
        moves.retain(|positions| {
            let len = positions.len();
            let mut sorted = positions.clone();
            sorted.sort_unstable();
            sorted.dedup();
            len == sorted.len()
        });
        let mut positions = moves
            .iter()
            .map(|positions| *positions.last().expect("Could not get last piece"))
            .collect::<Vec<Position>>();
        positions.sort_unstable();
        positions.dedup();
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, piece::Piece};

    #[test]
    fn tests_available_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Pillbug, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        let moves = Bug::available_moves(Position::new(0, 0), &board);
        assert_eq!(moves.get(&Position::new(0, 0)).unwrap().len(), 2);
        let moves = Bug::available_moves(Position::new(1, 0), &board);
        assert_eq!(moves.get(&Position::new(1, 0)).unwrap().len(), 6);
    }

    #[test]
    fn tests_available_abilities() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Pillbug, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        let positions = Bug::available_abilities(Position::new(0, 0), &board);
        assert_eq!(positions.get(&Position::new(1, 0)).unwrap().len(), 5);
        let positions = Bug::available_abilities(Position::new(1, 0), &board);
        assert_eq!(positions.get(&Position::new(0, 0)).unwrap().len(), 5);
    }

    #[test]
    fn tests_pillbug_throw() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Pillbug, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        let positions = Bug::pillbug_throw(Position::new(0, 0), &board);
        assert_eq!(positions.get(&Position::new(1, 0)).unwrap().len(), 5);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Pillbug, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        let positions = Bug::pillbug_throw(Position::new(0, 0), &board);
        assert!(!positions.contains_key(&Position::new(1, 0)));
    }

    #[test]
    fn tests_pillbug_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Pillbug, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        let positions = Bug::pillbug_moves(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn tests_mosquito_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Mosquito, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        let positions = Bug::mosquito_moves(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 0);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Mosquito, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        let positions = Bug::mosquito_moves(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 5);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Mosquito, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Pillbug, Color::Black, 0),
        );
        let positions = Bug::mosquito_moves(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 2);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        let positions = Bug::mosquito_moves(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 6);
    }

    #[test]
    fn tests_descend() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Ant, Color::White, 1),
        );
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Ant, Color::White, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        let positions = Bug::descend(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 3);
        assert!(positions.contains(&Position::new(-1, -1)));
        assert!(positions.contains(&Position::new(-1, 0)));
        assert!(positions.contains(&Position::new(-1, 1)));
    }

    #[test]
    fn tests_climb() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        let positions = Bug::climb(Position::new(1, 0), &board);
        assert_eq!(positions.len(), 1);
        assert!(positions.contains(&Position::new(0, 0)));

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        for (i, pos) in Position::new(0, 0).positions_around().enumerate() {
            board.insert(pos, Piece::new_from(Bug::Queen, Color::Black, 0));
            let positions = Bug::climb(Position::new(0, 0), &board);
            assert_eq!(positions.len(), i + 1);
        }

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        for pos in Position::new(0, 0).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Queen, Color::Black, 0));
        }
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Beetle, Color::White, 2),
        );
        let positions = Bug::climb(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 5);
    }

    #[test]
    fn tests_crawl() {
        // one neighbor gives 2 positions to move to
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position::new(0, -1)));
        assert!(positions.contains(&Position::new(0, 1)));

        // just a quick sanity check
        let mut board = Board::new();
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        for pos in Position::new(0, 1).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Queen, Color::Black, 0));
            let positions = Bug::crawl(Position::new(0, 1), &board).collect::<Vec<Position>>();
            assert_eq!(positions.len(), 2);
            board.board.remove(pos);
        }

        // two adjacent neighbors means two positions
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position::new(0, -1)));
        assert!(positions.contains(&Position::new(-1, 1)));

        // two (opposite) neighbors give 4 positions to move to
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(-1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 4);
        assert!(positions.contains(&Position::new(0, -1)));
        assert!(positions.contains(&Position::new(0, 1)));
        assert!(positions.contains(&Position::new(-1, -1)));
        assert!(positions.contains(&Position::new(-1, 1)));

        // two neighbors that form a gate
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(-1, 1),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position::new(0, -1)));
        assert!(positions.contains(&Position::new(-1, 0)));

        // a third neighbor forms a gate so we are back to 2 positions
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(-1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position::new(0, 1)));
        assert!(positions.contains(&Position::new(-1, 1)));

        // three neighbors that form a tripple gate means no movement
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 1),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(-1, 1),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(-1, -1),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 0);

        // three neighbors no gate -> 2 positions
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 1),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position::new(-1, -1)));
        assert!(positions.contains(&Position::new(-1, 1)));

        // four neighbors no gate -> 2 positions
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 1),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        board.insert(
            Position::new(-1, 1),
            Piece::new_from(Bug::Ladybug, Color::Black, 1),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position::new(-1, -1)));
        assert!(positions.contains(&Position::new(-1, 0)));

        // five neighbors -> 0 positions
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Ant, Color::Black, 2),
        );
        board.insert(
            Position::new(0, 1),
            Piece::new_from(Bug::Ant, Color::Black, 3),
        );
        board.insert(
            Position::new(-1, 1),
            Piece::new_from(Bug::Ladybug, Color::Black, 0),
        );
        board.insert(
            Position::new(-1, 0),
            Piece::new_from(Bug::Ladybug, Color::Black, 0),
        );
        let positions = Bug::crawl(Position::new(0, 0), &board).collect::<Vec<_>>();
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn tests_queen_moves() {
        tests_crawl()
    }

    #[test]
    fn tests_spider_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Spider, Color::White, 1),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Ant, Color::Black, 1),
        );
        let positions = Bug::spider_moves(Position::new(0, 0), &board);
        assert_eq!(positions.len(), 1);
        assert!(positions.contains(&Position::new(2, 0)));
    }

    #[test]
    fn tests_ladybug_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Ladybug, Color::White, 0),
        );
        board.insert(
            Position::new(-1, 0),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(-2, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        assert_eq!(Bug::ladybug_moves(Position::new(0, 0), &board).len(), 5);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Ladybug, Color::White, 0),
        );
        for pos in Position::new(0, 0).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Grasshopper, Color::Black, 1));
        }
        board.board.remove(Position::new(1, 0));
        assert_eq!(Bug::ladybug_moves(Position::new(0, 0), &board).len(), 12);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Ladybug, Color::White, 0),
        );
        for pos in Position::new(0, 0).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Grasshopper, Color::Black, 1));
        }
        board.insert(
            Position::new(-2, 0),
            Piece::new_from(Bug::Grasshopper, Color::Black, 1),
        );
        board.board.remove(Position::new(1, 0));
        assert_eq!(Bug::ladybug_moves(Position::new(0, 0), &board).len(), 14);
    }

    #[test]
    fn tests_beetle_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        board.insert(
            Position::new(0, -1),
            Piece::new_from(Bug::Queen, Color::White, 0),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Mosquito, Color::Black, 0),
        );
        assert_eq!(Bug::beetle_moves(Position::new(0, 0), &board).len(), 4);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        for pos in Position::new(0, 0).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Grasshopper, Color::White, 1));
        }
        assert_eq!(Bug::beetle_moves(Position::new(0, 0), &board).len(), 6);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        for pos in Position::new(0, 0).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Grasshopper, Color::White, 1));
        }
        board.board.remove(Position::new(1, 0));
        assert_eq!(Bug::beetle_moves(Position::new(0, 0), &board).len(), 5);
    }

    #[test]
    fn tests_ant_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Ant, Color::White, 1),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Beetle, Color::White, 1),
        );
        assert_eq!(
            board
                .neighbors(Position { x: 0, y: 0 })
                .last()
                .unwrap()
                .top_piece()
                .unwrap(),
            Piece::new_from(Bug::Beetle, Color::White, 1)
        );
        assert_eq!(Bug::ant_moves(Position::new(0, 0), &board).len(), 5);
    }

    #[test]
    fn tests_grasshopper_moves() {
        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Grasshopper, Color::White, 1),
        );
        for pos in Position::new(0, 0).positions_around() {
            board.insert(pos, Piece::new_from(Bug::Beetle, Color::White, 1));
        }
        assert_eq!(Bug::grasshopper_moves(Position::new(0, 0), &board).len(), 6);

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Grasshopper, Color::White, 1),
        );
        board.insert(
            Position::new(1, 0),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        board.insert(
            Position::new(3, 0),
            Piece::new_from(Bug::Beetle, Color::Black, 1),
        );
        assert_eq!(Bug::grasshopper_moves(Position::new(0, 0), &board).len(), 1);
        assert_eq!(
            *Bug::grasshopper_moves(Position::new(0, 0), &board)
                .last()
                .unwrap(),
            Position::new(2, 0)
        );

        let mut board = Board::new();
        board.insert(
            Position::new(0, 0),
            Piece::new_from(Bug::Grasshopper, Color::White, 1),
        );
        assert_eq!(Bug::grasshopper_moves(Position::new(0, 0), &board).len(), 0);
    }
}
