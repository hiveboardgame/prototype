use crate::bug::Bug;
use crate::color::Color;
use crate::game_error::GameError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Hash, Eq, Copy)]
pub struct Piece {
    pub bug: Bug,
    pub color: Color,
    pub order: Option<i8>,
}

impl FromStr for Piece {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c_chars) = s.chars().next() {
            let color = Color::from_str(&c_chars.to_string())?;
            if let Some(b_chars) = s.chars().nth(1) {
                let bug = Bug::from_str(&b_chars.to_string())?;
                let mut order = None;
                if let Some(ch) = s.chars().nth(2) {
                    if let Ok(ord) = ch.to_string().parse() {
                        order = Some(ord)
                    }
                }
                return Ok(Piece::new(bug, color, order));
            }
        }
        return Err(GameError::ParsingError {
            found: s.to_string(),
            typ: "piece".to_string(),
        });
    }
}

impl Piece {
    pub fn new(bug: Bug, color: Color, order: Option<i8>) -> Piece {
        Piece { bug, color, order }
    }

    pub fn is_color(&self, color: &Color) -> bool {
        *color == self.color
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(order) = self.order {
            write!(f, "{}{}{}", self.color, self.bug, order)
        } else {
            write!(f, "{}{} ", self.color, self.bug)
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(order) = self.order {
            write!(f, "{}{}{}", self.color, self.bug, order)
        } else {
            write!(f, "{}{} ", self.color, self.bug)
        }
    }
}
