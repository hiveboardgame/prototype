use crate::game_error::GameError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::White => "white",
        }
    }

    pub fn from_str(s: &str) -> Result<Color, GameError> {
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            any => Err(GameError::ParsingError {
                found: any.to_string(),
                typ: "color string".to_string(),
            }),
        }
    }

    pub fn to_html_color(&self) -> &'static str {
        match self {
            Self::Black => "#131200",
            Self::White => "#F0EAD6",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::White => "w",
            Color::Black => "b",
        };
        write!(f, "{}", color)
    }
}
