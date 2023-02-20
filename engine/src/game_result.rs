use crate::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameResult {
    Winner(Color),
    Draw,
    Unknown,
}

impl Default for GameResult {
    fn default() -> Self {
        GameResult::Unknown
    }
}

