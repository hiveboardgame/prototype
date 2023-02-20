use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{board::Board, piece::Piece};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Position(pub i8, pub i8);

pub enum Direction {
    NW,
    NE,
    E,
    SE,
    SW,
    W,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::NW => write!(f, "NorthWest"),
            Direction::NE => write!(f, "NorthEast"),
            Direction::E => write!(f, "East"),
            Direction::SE => write!(f, "SouthEast"),
            Direction::SW => write!(f, "SouthWest"),
            Direction::W => write!(f, "West"),
        }
    }
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::NW,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
        ]
    }

    pub fn adjacent_directions(&self) -> (Direction, Direction) {
        match self {
            Direction::NW => (Direction::W, Direction::NE),
            Direction::NE => (Direction::NW, Direction::E),
            Direction::E => (Direction::NE, Direction::SE),
            Direction::SE => (Direction::E, Direction::SW),
            Direction::SW => (Direction::SE, Direction::W),
            Direction::W => (Direction::SW, Direction::NW),
        }
    }

    pub fn to_history_string(&self, piece: String) -> String {
        match self {
            Direction::NE => piece + "/",
            Direction::E => piece + "-",
            Direction::SE => piece + "\\",
            Direction::NW => "\\".to_string() + &piece,
            Direction::SW => "/".to_string() + &piece,
            Direction::W => "-".to_string() + &piece,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.0, self.1)
    }
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Self(x, y)
    }

    // this implements "odd-r horizontal" which offsets odd rows to the right
    pub fn direction(&self, to: &Position) -> Direction {
        // even rows
        if self.1.rem_euclid(2) == 0 {
            return match (to.0 - self.0, to.1 - self.1) {
                (-1, -1) => Direction::NW,
                (0, -1) => Direction::NE,
                (1, 0) => Direction::E,
                (0, 1) => Direction::SE,
                (-1, 1) => Direction::SW,
                (-1, 0) => Direction::W,
                (x, y) => panic!(
                    "(even) Direction of movement unknown, from: {} to: {} ({x},{y})",
                    self, to
                ),
            };
        }
        // odd rows
        match (to.0 - self.0, to.1 - self.1) {
            (0, -1) => Direction::NW,
            (1, -1) => Direction::NE,
            (1, 0) => Direction::E,
            (1, 1) => Direction::SE,
            (0, 1) => Direction::SW,
            (-1, 0) => Direction::W,
            (x, y) => panic!(
                "(odd) Direction of movement unknown, from: {} to: {} ({x},{y})",
                self, to
            ),
        }
    }

    pub fn common_adjacent_positions(&self, to: &Position) -> (Position, Position) {
        let (dir1, dir2) = self.direction(to).adjacent_directions();
        (self.to(&dir1), self.to(&dir2))
    }

    pub fn to(&self, direction: &Direction) -> Position {
        // even rows
        if self.1.rem_euclid(2) == 0 {
            return match direction {
                Direction::NW => Position(self.0 - 1, self.1 - 1),
                Direction::NE => Position(self.0, self.1 - 1),
                Direction::E => Position(self.0 + 1, self.1),
                Direction::SE => Position(self.0, self.1 + 1),
                Direction::SW => Position(self.0 - 1, self.1 + 1),
                Direction::W => Position(self.0 - 1, self.1),
            };
        }
        // odd rows
        match direction {
            Direction::NW => Position(self.0, self.1 - 1),
            Direction::NE => Position(self.0 + 1, self.1 - 1),
            Direction::E => Position(self.0 + 1, self.1),
            Direction::SE => Position(self.0 + 1, self.1 + 1),
            Direction::SW => Position(self.0, self.1 + 1),
            Direction::W => Position(self.0 - 1, self.1),
        }
    }

    pub fn from_string(s: &str, board: &Board) -> Position {
        if s.starts_with('.') {
            return Position(0, 0);
        }

        let re = Regex::new(r"([-/\\]?)([wb][ABGMLPSQ]\d?)([-/\\]?)").unwrap();
        let cap = re.captures(s).unwrap();
        let piece = Piece::from_string(&cap[2]);
        let mut position = board.position(&piece);
        if !cap[1].is_empty() {
            match &cap[1] {
                "\\" => {
                    position = position.to(&Direction::NW);
                }
                "-" => {
                    position = position.to(&Direction::W);
                }
                "/" => {
                    position = position.to(&Direction::SW);
                }
                _ => {
                    panic!("Not a valid direction");
                }
            }
        }
        if !cap[3].is_empty() {
            match &cap[3] {
                "/" => {
                    position = position.to(&Direction::NE);
                }
                "-" => {
                    position = position.to(&Direction::E);
                }
                "\\" => {
                    position = position.to(&Direction::SE);
                }
                _ => {
                    panic!("Not a valid direction");
                }
            }
        }
        position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_direction_and_to() {
        for position in [Position(0, 0), Position(0, 1)] {
            for direction in Direction::all() {
                let new_position = position.to(&direction);
                let opposite_direction = new_position.direction(&position);
                let inital_position = new_position.to(&opposite_direction);
                assert_eq!(position, inital_position);
            }
        }
    }
}
