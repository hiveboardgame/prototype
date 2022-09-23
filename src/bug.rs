use std::collections::HashMap;
use std::fmt;

use crate::board::{self, Board};
use crate::position::Position;

// https://entomology.gitlab.io/notation.html
// Each piece used during a game has a unique name. This name is made of one lowercase letter for
// the color (w for white or b for black) followed by one uppercase letter for the bug type (A for
// Ant, B for Beetle, G for Grasshopper, L for Ladybug, M for Mosquito, P for Pillbug, Q for Queen
// Bee or S for Spider). In case the bug appears multiple times, the piece name also contains one
// digit indicating in which order that piece came into play. For instance, wA1 is the first white
// Ant that has been added to the hive while bB2 is the second black Beetle. Since there is only
// one copy of it, the white Pillbug is simply named wP and not wP1.
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
        return bugs;
    }

    pub fn available_moves(bug: Bug, board: &Board) -> Vec<Position> {
        return match bug {
            Bug::Ant => unimplemented!(),
            Bug::Beetle => unimplemented!(),
            Bug::Grasshopper => unimplemented!(),
            Bug::Ladybug => unimplemented!(),
            Bug::Mosquito => unimplemented!(),
            Bug::Pillbug => unimplemented!(),
            Bug::Queen => Bug::queen_moves(board),
            Bug::Spider => unimplemented!(),
        }
    }

    fn queen_moves(board: &Board) -> Vec<Position> {
        unimplemented!();
    }
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
