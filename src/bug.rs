use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::board::Board;
use crate::position::{self, Direction, Position};

// https://entomology.gitlab.io/notation.html
// Each piece used during a game has a unique name. This name is made of one lowercase letter for
// the color (w for white or b for black) followed by one uppercase letter for the bug type (A for
// Ant, B for Beetle, G for Grasshopper, L for Ladybug, M for Mosquito, P for Pillbug, Q for Queen
// Bee or S for Spider). In case the bug appears multiple times, the piece name also contains one
// digit indicating in which order that piece came into play. For instance, wA1 is the first white
// Ant that has been added to the hive while bB2 is the second black Beetle. Since there is only
// one copy of it, the white Pillbug is simply named wP and not wP1.
//
//
// Bugs can:
// *crawl* which means they stay on their initial level
// *climb* which means they increase their level to what ever is 1 higher than the level already
// taken
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Bug {
    Ant,
    Beetle,
    Grasshopper,
    Ladybug,
    Mosquito,
    Pillbug,
    Queen,
    Spider,
}

impl fmt::Display for Bug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = match self {
            Bug::Ant => "A",
            Bug::Beetle => "B",
            Bug::Grasshopper => "G",
            Bug::Ladybug => "L",
            Bug::Mosquito => "M",
            Bug::Pillbug => "P",
            Bug::Queen => "Q",
            Bug::Spider => "S",
        };
        write!(f, "{}", b)
    }
}

impl Bug {
    pub fn all() -> HashMap<Bug, i8> {
        let mut bugs = HashMap::new();
        bugs.insert(Bug::Ant, 3);
        bugs.insert(Bug::Beetle, 2);
        bugs.insert(Bug::Grasshopper, 3);
        bugs.insert(Bug::Ladybug, 1);
        bugs.insert(Bug::Mosquito, 1);
        bugs.insert(Bug::Pillbug, 1);
        bugs.insert(Bug::Queen, 1);
        bugs.insert(Bug::Spider, 3);
        bugs
    }

    pub fn available_moves(position: &Position, board: &Board) -> Vec<Position> {
        if board.pinned(position) {
            return vec![];
        }
        let bug = board.board.get(position).unwrap().last().unwrap().bug;
        return match bug {
            Bug::Ant => unimplemented!(),
            Bug::Beetle => Bug::beetle_moves(position, board),
            Bug::Grasshopper => Bug::grasshopper_moves(position, board),
            Bug::Ladybug => Bug::ladybug_moves(position, board),
            Bug::Mosquito => unimplemented!(),
            Bug::Pillbug => unimplemented!(),
            Bug::Queen => Bug::queen_moves(position, board),
            Bug::Spider => unimplemented!(),
        };
    }

    pub fn grasshopper_moves(position: &Position, board: &Board) -> Vec<Position> {
        // get the directions of the grasshopper's neighbors
        let direction_of_neighbors = board
            .positions_taken_around(position)
            .iter()
            .map(|pos| position.direction(pos))
            .collect::<Vec<Direction>>();
        let mut positions = vec![];
        // move in the given direction
        for dir in direction_of_neighbors.iter() {
            let mut cur_pos = position.clone();
            // until there is a free position
            while let Some(_) = board.board.get(&cur_pos.to(dir)) {
                cur_pos = cur_pos.to(dir);
            }
            // then add the free position
            positions.push(cur_pos.to(dir));
        }
        return positions;
    }

    pub fn beetle_moves(position: &Position, board: &Board) -> Vec<Position> {
        let level = board
            .board
            .get(position)
            .expect(&format!("There is no beetle at pos: {}", position))
            .len();
        // get free positions around beetle
        let free_around_beetle: HashSet<Position> =
            HashSet::from_iter(board.positions_available_around(position));
        // the positions that are taken, beetles can climb on them
        let taken_around_beetle: HashSet<Position> =
            HashSet::from_iter(board.positions_taken_around(position));
        // get the positions that are free around the neighbors
        let free_around_neighbors = HashSet::from_iter(
            board
                .positions_taken_around(position)
                .iter()
                .flat_map(|pos| board.positions_available_around(pos)),
        );
        // intersect to find connected positions that the beetle can move on without climbing
        free_around_beetle
            .intersection(&free_around_neighbors)
            // filter out the positions we cannot crawl to
            .filter(|pos| !board.gated(level, position, pos))
            .cloned()
            .collect::<HashSet<Position>>()
            // add the climbable positions
            .union(&taken_around_beetle)
            .cloned()
            .collect()
    }

    fn crawl(position: &Position, board: &Board) -> Vec<Position> {
        let occupied = board.positions_taken_around(position);
        occupied
            .iter()
            .flat_map(|pos| {
                let mut positions = vec![];
                let (pos1, pos2) = position.common_adjacent_positions(pos);
                if !board.gated(1, position, &pos1) && !occupied.contains(&pos1) {
                    positions.push(pos1);
                }
                if !board.gated(1, position, &pos2) && !occupied.contains(&pos2) {
                    positions.push(pos2);
                }
                positions
            })
            .collect()
    }

    fn ladybug_moves(position: &Position, board: &Board) -> Vec<Position> {
        // find all adjacent bugs to climb on
        let climb = board.positions_taken_around(position);
        // get all bugs around the
        let climb_crawl = climb
            .iter()
            .flat_map(|pos| board.positions_taken_around(pos))
            .filter(|pos| pos != position)
            .collect::<HashSet<Position>>();
        let crawl = climb_crawl
            .iter()
            .flat_map(|from| {
                board
                    .positions_available_around(&from)
                    .iter()
                    .filter(|to| {
                        board.gated(
                            board
                                .board
                                .get(&from)
                                .expect("Failed to get bug to compute level height")
                                .len(),
                            from,
                            to,
                        )
                    })
                    .cloned()
                    .collect::<Vec<Position>>()
            })
            .filter(|pos| pos != position && !climb.contains(pos) && !climb_crawl.contains(pos))
            .collect::<HashSet<Position>>();
        return crawl.iter().cloned().collect();
    }

    fn queen_moves(position: &Position, board: &Board) -> Vec<Position> {
        Bug::crawl(position, board)
    }

    fn spider_moves(position: &Position, board: &Board) -> Vec<Position> {
        let first: HashSet<Position> = HashSet::from_iter(Bug::crawl(position, board));
        println!("{:?}", first);
        println!("{}", board);
        let mut board = board.clone();
        board.board.remove(position);
        println!("{}", board);
        let second: HashSet<Position> = first
            .iter()
            .flat_map(|pos| {
                Bug::crawl(pos, &board)
                    .iter()
                    .filter(|pos| *pos != position && !first.contains(pos))
                    .cloned()
                    .collect::<HashSet<Position>>()
            })
            .collect::<HashSet<Position>>();
        println!("{:?}", second);
        let third: Vec<Position> = second
            .iter()
            .flat_map(|pos| {
                println!("{pos}");
                println!("{}", board);
                Bug::crawl(pos, &board)
                    .iter()
                    .filter(|pos| *pos != position && !first.contains(pos) && !second.contains(pos))
                    .cloned()
                    .collect::<HashSet<Position>>()
            })
            .collect::<Vec<Position>>();
        println!("{:?}", third);
        return third;
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, piece::Piece};

    use super::*;

    #[test]
    fn tests_crawl() {
        // one neighbor gives 2 positions to move to
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position(0, -1)));
        assert!(positions.contains(&Position(0, 1)));

        // just a quick sanity check
        let mut board = Board::new();
        board.spawn(&Position(0, 1), Piece::new(Bug::Queen, Color::White, 1));
        for pos in board.positions_around(&Position(0, 1)).iter() {
            board.spawn(pos, Piece::new(Bug::Queen, Color::Black, 1));
            let positions = Bug::crawl(&Position(0, 1), &board);
            println!("{:?}",positions);
            assert_eq!(positions.len(), 2);
            board.board.remove(pos);
        }

        // two adjacent neighbors means two positions
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(0, 1), Piece::new(Bug::Ant, Color::Black, 2));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position(0, -1)));
        assert!(positions.contains(&Position(-1, 1)));

        // two (opposite) neighbors give 4 positions to move to
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(-1, 0), Piece::new(Bug::Ant, Color::Black, 2));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 4);
        assert!(positions.contains(&Position(0, -1)));
        assert!(positions.contains(&Position(0, 1)));
        assert!(positions.contains(&Position(-1, -1)));
        assert!(positions.contains(&Position(-1, 1)));

        // two neighbors that form a gate
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(-1, 1), Piece::new(Bug::Ant, Color::Black, 2));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position(0, -1)));
        assert!(positions.contains(&Position(-1, 0)));

        // a third neighbor forms a gate so we are back to 2 positions
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(-1, 0), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(0, -1), Piece::new(Bug::Ant, Color::Black, 3));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position(0, 1)));
        assert!(positions.contains(&Position(-1, 1)));

        // three neighbors that form a tripple gate means no movement
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(-1, 1), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(-1, -1), Piece::new(Bug::Ant, Color::Black, 3));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 0);

        // three neighbors no gate -> 2 positions
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(0, -1), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(0, 1), Piece::new(Bug::Ant, Color::Black, 3));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position(-1, -1)));
        assert!(positions.contains(&Position(-1, 1)));

        // four neighbors no gate -> 2 positions
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(0, -1), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(0, 1), Piece::new(Bug::Ant, Color::Black, 3));
        board.spawn(&Position(-1, 1), Piece::new(Bug::Ladybug, Color::Black, 1));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Position(-1, -1)));
        assert!(positions.contains(&Position(-1, 0)));

        // five neighbors -> 0 positions
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        board.spawn(&Position(0, -1), Piece::new(Bug::Ant, Color::Black, 2));
        board.spawn(&Position(0, 1), Piece::new(Bug::Ant, Color::Black, 3));
        board.spawn(&Position(-1, 1), Piece::new(Bug::Ladybug, Color::Black, 1));
        board.spawn(&Position(-1, 0), Piece::new(Bug::Ladybug, Color::Black, 1));
        let positions = Bug::crawl(&Position(0, 0), &board);
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn tests_queen_moves() {
        tests_crawl()
    }

    #[test]
    fn tests_spider_moves() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Spider, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Ant, Color::Black, 1));
        let positions = Bug::spider_moves(&Position(0, 0), &board);
        println!("{:?}", positions);
        assert_eq!(positions.len(), 1);
    }


    #[test]
    fn tests_ladybug_moves() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Ladybug, Color::White, 1));
        board.spawn(&Position(-1, 0), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(-2, 0), Piece::new(Bug::Mosquito, Color::Black, 1));
        assert_eq!(Bug::ladybug_moves(&Position(0, 0), &board).len(), 5);

        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Ladybug, Color::White, 1));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Grasshopper, Color::Black, 1));
        }
        board.board.remove(&Position(1, 0));
        assert_eq!(Bug::ladybug_moves(&Position(0, 0), &board).len(), 12);

        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Ladybug, Color::White, 1));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Grasshopper, Color::Black, 1));
        }
        board.spawn(
            &Position(-2, 0),
            Piece::new(Bug::Grasshopper, Color::Black, 1),
        );
        board.board.remove(&Position(1, 0));
        assert_eq!(Bug::ladybug_moves(&Position(0, 0), &board).len(), 14);
    }

    #[test]
    fn tests_beetle_moves() {
        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Beetle, Color::White, 1));
        board.spawn(&Position(0, -1), Piece::new(Bug::Queen, Color::White, 1));
        board.spawn(&Position(1, 0), Piece::new(Bug::Mosquito, Color::Black, 1));
        assert_eq!(Bug::beetle_moves(&Position(0, 0), &board).len(), 4);

        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Beetle, Color::White, 1));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Grasshopper, Color::White, 1));
        }
        assert_eq!(Bug::beetle_moves(&Position(0, 0), &board).len(), 6);

        let mut board = Board::new();
        board.spawn(&Position(0, 0), Piece::new(Bug::Beetle, Color::White, 1));
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Grasshopper, Color::White, 1));
        }
        board.board.remove(&Position(1, 0));
        assert_eq!(Bug::beetle_moves(&Position(0, 0), &board).len(), 5);
    }

    #[test]
    fn tests_grasshopper_moves() {
        let mut board = Board::new();
        board.spawn(
            &Position(0, 0),
            Piece::new(Bug::Grasshopper, Color::White, 1),
        );
        for pos in board.positions_around(&Position(0, 0)).iter() {
            board.spawn(pos, Piece::new(Bug::Beetle, Color::White, 1));
        }
        assert_eq!(Bug::grasshopper_moves(&Position(0, 0), &board).len(), 6);

        let mut board = Board::new();
        board.spawn(
            &Position(0, 0),
            Piece::new(Bug::Grasshopper, Color::White, 1),
        );
        board.spawn(&Position(1, 0), Piece::new(Bug::Beetle, Color::Black, 1));
        board.spawn(&Position(3, 0), Piece::new(Bug::Beetle, Color::Black, 1));
        assert_eq!(Bug::grasshopper_moves(&Position(0, 0), &board).len(), 1);
        assert_eq!(
            *Bug::grasshopper_moves(&Position(0, 0), &board)
                .last()
                .unwrap(),
            Position(2, 0)
        );

        let mut board = Board::new();
        board.spawn(
            &Position(0, 0),
            Piece::new(Bug::Grasshopper, Color::White, 1),
        );
        assert_eq!(Bug::grasshopper_moves(&Position(0, 0), &board).len(), 0);
    }
}
