use crate::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum GameResult {
    Winner(Color),
    Draw,
    #[default]
    Unknown,
}


