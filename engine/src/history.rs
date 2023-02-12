use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};

use crate::color::Color;
use crate::game_result::GameResult;

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq)]
pub struct History {
    pub moves: Vec<(String, String)>,
    pub result: GameResult,
}

impl History {
    pub fn new() -> Self {
        History {
            moves: Vec::new(),
            result: GameResult::Unknown,
        }
    }

    pub fn to_string(&self) -> String {
        let mut his = String::new();
        for (i, (piece, pos)) in self.moves.iter().enumerate() {
            his += &format!("{}. {} {}", i + 1, piece, pos);
        }
        his
    }

    pub fn record_move(&mut self, piece: String, pos: String) {
        self.moves.push((piece, pos));
    }

    pub fn from_filepath(str: &str) -> Self {
        let mut history = History::new();
        let header = Regex::new(r"\[.*").unwrap();
        let turn = Regex::new(r"\d+").unwrap();
        let result = Regex::new(r"\[Result").unwrap();
        if let Ok(file) = File::open(str) {
            for line in io::BufReader::new(file).lines().flatten() {
                let tokens = line.split_whitespace().collect::<Vec<&str>>();
                if line.len() == 0 {
                    continue;
                }
                if result.is_match(tokens.first().unwrap()) {
                    match tokens.get(1) {
                        Some(&"\"1-0\"]") => history.result = GameResult::Winner(Color::White),
                        Some(&"\"0-1\"]") => history.result = GameResult::Winner(Color::Black),
                        Some(&"\"1/2-1/2\"]") => history.result = GameResult::Draw,
                        _ => history.result = GameResult::Unknown,
                    }
                }
                if header.is_match(tokens.first().unwrap()) {
                    continue;
                }
                if turn.is_match(tokens.first().unwrap()) {
                    if history.moves.is_empty() && tokens.get(2).is_none() {
                        history
                            .moves
                            .push((tokens.get(1).unwrap().to_string(), ".".to_string()));
                    } else {
                        history.moves.push((
                            tokens.get(1).unwrap().to_string(),
                            tokens.get(2).unwrap_or(&"").to_string(),
                        ));
                    }
                }
            }
        }
        history
    }

    pub fn write_file(&self, string: String) {
        if let Ok(_file) = File::open(string) {
            for _mov in self.moves.iter() {
                // TODO: file write
            }
        }
    }
}
