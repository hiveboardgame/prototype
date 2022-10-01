use std::fmt;

use crate::bug::Bug;
use crate::color::Color;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Piece {
    pub bug: Bug,
    pub order: i8,
    pub color: Color,
}

impl Piece {
    pub fn new(bug: Bug, color: Color, order: i8) -> Piece {
        Piece { bug, color, order }
    }

    pub fn is_color(&self, color: Color) -> bool {
        color == self.color
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.color, self.bug, self.order)
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.color, self.bug, self.order)
    }
}
