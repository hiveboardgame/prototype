use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    board::{Board, BOARD_SIZE},
    direction::Direction,
    game_error::GameError,
    piece::Piece,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        let x = x.rem_euclid(BOARD_SIZE as i8);
        let y = y.rem_euclid(BOARD_SIZE as i8);
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn new_i32(x: i32, y: i32) -> Self {
        let x = x.rem_euclid(BOARD_SIZE);
        let y = y.rem_euclid(BOARD_SIZE);
        Self { x, y }
    }

    pub fn new_i8(x: i8, y: i8) -> Self {
        let x = x.rem_euclid(BOARD_SIZE as i8);
        let y = y.rem_euclid(BOARD_SIZE as i8);
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn inital_spawn_position() -> Self {
        // TODO make this depenedent on BOARD_SIZE
        Self { x: 15, y: 15 }
    }

    fn wrap_around(num: i32) -> i32 {
        if num == (BOARD_SIZE - 1) {
            return -1;
        }
        if num == (-(BOARD_SIZE - 1)) {
            return 1;
        }
        return num;
    }

    // this implements "odd-r horizontal" which offsets odd rows to the right
    pub fn direction(&self, to: Position) -> Direction {
        let diff = (
            Self::wrap_around(to.x - self.x),
            Self::wrap_around(to.y - self.y),
        );
        // even rows
        if self.y.rem_euclid(2) == 0 {
            return match diff {
                (-1, -1) => Direction::NW,
                (0, -1) => Direction::NE,
                (1, 0) => Direction::E,
                (0, 1) => Direction::SE,
                (-1, 1) => Direction::SW,
                (-1, 0) => Direction::W,
                // This panic is okay, because if it ever gets called with an invalid move, it
                // implies there is a problem with the engine itself, not with user input
                (x, y) => {
                    panic!("(even) Direction of movement unknown, from: {self} to: {to} ({x},{y})")
                }
            };
        }
        // odd rows
        match diff {
            (0, -1) => Direction::NW,
            (1, -1) => Direction::NE,
            (1, 0) => Direction::E,
            (1, 1) => Direction::SE,
            (0, 1) => Direction::SW,
            (-1, 0) => Direction::W,
            // This panic is okay, because if it ever gets called with an invalid move, it
            // implies there is a problem with the engine itself, not with user input
            (x, y) => {
                panic!("(odd) Direction of movement unknown, from: {self} to: {to} ({x},{y})")
            }
        }
    }

    pub fn common_adjacent_positions(&self, to: Position) -> (Position, Position) {
        let (dir1, dir2) = self.direction(to).adjacent_directions();
        (self.to(&dir1), self.to(&dir2))
    }

    // pub fn positions_around(&self) -> impl Iterator<Item = Position> + '_  {
    //     // TODO this can be done statically
    //     static DIRS: [Direction; 6] = [
    //         Direction::NW,
    //         Direction::NE,
    //         Direction::E,
    //         Direction::SE,
    //         Direction::SW,
    //         Direction::W,
    //     ];
    //     DIRS.iter().map(move |dir| self.to(dir))
    // }

    pub fn positions_around(&self) -> impl Iterator<Item = Position> {
        // even rows
        if self.y.rem_euclid(2) == 0 {
            return [
                Position::new_i32(self.x - 1, self.y - 1),
                Position::new_i32(self.x, self.y - 1),
                Position::new_i32(self.x + 1, self.y),
                Position::new_i32(self.x, self.y + 1),
                Position::new_i32(self.x - 1, self.y + 1),
                Position::new_i32(self.x - 1, self.y),
            ]
            .into_iter();
        }
        // odd rows
        [
            Position::new_i32(self.x, self.y - 1),
            Position::new_i32(self.x + 1, self.y - 1),
            Position::new_i32(self.x + 1, self.y),
            Position::new_i32(self.x + 1, self.y + 1),
            Position::new_i32(self.x, self.y + 1),
            Position::new_i32(self.x - 1, self.y),
        ]
        .into_iter()
    }

    pub fn to(&self, direction: &Direction) -> Position {
        // even rows
        if self.y.rem_euclid(2) == 0 {
            return match direction {
                Direction::NW => Position::new_i32(self.x - 1, self.y - 1),
                Direction::NE => Position::new_i32(self.x, self.y - 1),
                Direction::E => Position::new_i32(self.x + 1, self.y),
                Direction::SE => Position::new_i32(self.x, self.y + 1),
                Direction::SW => Position::new_i32(self.x - 1, self.y + 1),
                Direction::W => Position::new_i32(self.x - 1, self.y),
            };
        }
        // odd rows
        match direction {
            Direction::NW => Position::new_i32(self.x, self.y - 1),
            Direction::NE => Position::new_i32(self.x + 1, self.y - 1),
            Direction::E => Position::new_i32(self.x + 1, self.y),
            Direction::SE => Position::new_i32(self.x + 1, self.y + 1),
            Direction::SW => Position::new_i32(self.x, self.y + 1),
            Direction::W => Position::new_i32(self.x - 1, self.y),
        }
    }

    pub fn from_string(s: &str, board: &Board) -> Result<Position, GameError> {
        if s.starts_with('.') {
            return Ok(Position::inital_spawn_position());
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"([-/\\]?)([wb][ABGMLPSQ]\d?)([-/\\]?)")
                .expect("This regex should compile");
        }
        if let Some(cap) = RE.captures(s) {
            let piece: Piece = cap[2].parse()?;
            if let Some(mut position) = board.position_of_piece(piece) {
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
                        any => {
                            return Err(GameError::InvalidDirection {
                                direction: any.to_string(),
                            })
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
                        any => {
                            return Err(GameError::InvalidDirection {
                                direction: any.to_string(),
                            })
                        }
                    }
                }
                return Ok(position);
            }
        }
        Err(GameError::ParsingError {
            found: s.to_string(),
            typ: "position".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_direction_and_to() {
        for position in [Position::new(0, 0), Position::new(0, 1)] {
            for direction in Direction::all() {
                let new_position = position.to(&direction);
                let opposite_direction = new_position.direction(position);
                let inital_position = new_position.to(&opposite_direction);
                assert_eq!(position, inital_position);
            }
        }
    }

    #[test]
    fn tests_direction_and_to_circles() {
        use Direction::*;
        let pos = Position::new(0, 0);
        let nw = pos.to(&Direction::NW);
        let sw = nw.to(&Direction::SW);
        let e = sw.to(&Direction::E);
        assert_eq!(e, pos);
        let dirs = vec![NW, NW, SW, SW, E, E];
        let pos_0_0 = Position::new(0, 0);
        let mut pos = Position::new(0, 0);
        for direction in dirs {
            pos = pos.to(&direction)
        }
        assert_eq!(pos_0_0, pos);
        let dirs = vec![NW, SW, SE, E, NE, NW, SW];
        let pos_0_0 = Position::new(0, 0);
        let mut pos = Position::new(0, 0);
        for direction in dirs {
            pos = pos.to(&direction)
        }
        assert_eq!(pos_0_0, pos);
    }
}
