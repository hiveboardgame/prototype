use serde::{Deserialize, Serialize};

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
    pub fn from_str(str: &str) -> GameType {
        return match str {
            "Base" => GameType::Base,
            "Base+M" => GameType::M,
            "Base+L" => GameType::L,
            "Base+P" => GameType::P,
            "Base+ML" => GameType::ML,
            "Base+MP" => GameType::MP,
            "Base+LP" => GameType::LP,
            "Base+MLP" => GameType::MLP,
            _ => panic!("Unknown game string"),
        }
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
        }.to_string()
    }
}
