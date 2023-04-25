use crate::color::Color;
use crate::game_error::GameError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GameResult {
    Winner(Color),
    Draw,
    #[default]
    Unknown,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let game_result = match self {
            Self::Unknown => "Unknown".to_owned(),
            Self::Draw => "Draw".to_owned(),
            Self::Winner(color) => format!("Winner({color})"),
        };
        write!(f, "{game_result}")
    }
}

impl FromStr for GameResult {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unknown" => Ok(GameResult::Unknown),
            "Winner(black)" => Ok(GameResult::Winner(Color::Black)),
            "Winner(white)" => Ok(GameResult::Winner(Color::White)),
            "Draw" => Ok(GameResult::Draw),
            any => Err(GameError::ParsingError {
                found: any.to_string(),
                typ: "GameResult string".to_string(),
            }),
        }
    }
}
