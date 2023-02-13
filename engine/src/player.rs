use crate::{bug::Bug, game_type::GameType};
use crate::color::Color;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
