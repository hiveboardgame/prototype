use serde::{Deserialize, Serialize};

use crate::game_error::GameError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum GameType {
    Base,
    M,
    L,
    P,
    ML,
    LP,
    MP,
    MLP,
}

impl Default for GameType {
    fn default() -> Self {
        GameType::Base
    }
}

impl GameType {
    pub fn from_str(str: &str) -> Result<GameType, GameError> {
        return match str {
            "Base" => Ok(GameType::Base),
            "Base+M" => Ok(GameType::M),
            "Base+L" => Ok(GameType::L),
            "Base+P" => Ok(GameType::P),
            "Base+ML" => Ok(GameType::ML),
            "Base+MP" => Ok(GameType::MP),
            "Base+LP" => Ok(GameType::LP),
            "Base+MLP" => Ok(GameType::MLP),
            any => Err(GameError::ParsingError {
                found: any.to_string(),
                typ: "game type string".to_string(),
            }),
        };
    }

    pub fn to_string(&self) -> String {
        match self {
            GameType::Base => "Base",
            GameType::M => "Base+M",
            GameType::L => "Base+L",
            GameType::P => "Base+P",
            GameType::ML => "Base+ML",
            GameType::MP => "Base+MP",
            GameType::LP => "Base+LP",
            GameType::MLP => "Base+MLP",
        }
        .to_string()
    }
}
