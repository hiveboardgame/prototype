use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.0, self.1)
    }
}

impl Position {
    // this implements "odd-r horizontal" which offsets odd rows to the right
    pub fn direction(&self, to: &Position) -> Direction {
        // even rows
        if to.1.rem_euclid(2) == 0 {
            return match (to.0 - self.0, to.1 - self.1) {
                (-1, -1) => Direction::NW,
                (0, -1) => Direction::NE,
                (1, 0) => Direction::E,
                (0, 1) => Direction::SE,
                (-1, 1) => Direction::SW,
                (-1, 0) => Direction::W,
                (_, _) => panic!("Direction of movement unknown, from: {} to: {}", self, to),
            };
        }
        // odd rows
        return match (to.0 - self.0, to.1 - self.1) {
            (0, -1) => Direction::NW,
            (1, -1) => Direction::NE,
            (1, 0) => Direction::E,
            (-1, -1) => Direction::SE,
            (0, 1) => Direction::SW,
            (-1, 0) => Direction::W,
            (_, _) => panic!("Direction of movement unknown, from: {} to: {}", self, to),
        };
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
                Direction::NE => Position(self.0 + 0, self.1 - 1),
                Direction::E => Position(self.0 + 1, self.1 + 0),
                Direction::SE => Position(self.0 + 0, self.1 + 1),
                Direction::SW => Position(self.0 - 1, self.1 + 1),
                Direction::W => Position(self.0 - 1, self.1 + 0),
            };
        }
        // odd rows
        return match direction {
            Direction::NW => Position(self.0 + 0, self.1 - 1),
            Direction::NE => Position(self.0 + 1, self.1 - 1),
            Direction::E => Position(self.0 + 1, self.1 + 0),
            Direction::SE => Position(self.0 + 1, self.1 + 1),
            Direction::SW => Position(self.0 - 0, self.1 + 1),
            Direction::W => Position(self.0 - 1, self.1 + 0),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_direction_and_to() {
        for direction in Direction::all() {
            let position = Position(0, 0);
            println!("direction: {direction}");
            let new_position = position.to(&direction);
            println!("new_position: {new_position}");
            let opposite_direction = new_position.direction(&position);
            println!("opposite_direction: {opposite_direction}");
            let inital_position = new_position.to(&opposite_direction);
            println!("got to: {inital_position}");
            assert_eq!(position, inital_position);
        }
    }
}
