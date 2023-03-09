use crate::game_error::GameError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy, Debug, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl FromStr for Color {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            any => Err(GameError::ParsingError {
                found: any.to_string(),
                typ: "color string".to_string(),
            }),
        }
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
        write!(f, "{color}")
    }
}
