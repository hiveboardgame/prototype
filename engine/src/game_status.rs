use crate::game_result::GameResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum GameStatus {
    #[default]
    NotStarted,
    InProgress,
    Finished(GameResult),
}
