use crate::color::Color;
use crate::{bug::Bug, game_type::GameType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Player {
    color: Color,
    bugs: HashMap<Bug, i8>,
}

impl Player {
    pub fn new(color: Color) -> Player {
        Player {
            color,
            bugs: Bug::bugs_count(GameType::default()),
        }
    }
}
