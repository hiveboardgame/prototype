use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};

use crate::color::Color;
use crate::game_result::GameResult;
use crate::game_type::GameType;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq)]
pub struct History {
    pub moves: Vec<(String, String)>,
    pub result: GameResult,
    pub game_type: GameType,
}

impl History {
    pub fn new() -> Self {
        History {
            moves: Vec::new(),
            result: GameResult::Unknown,
            game_type: GameType::default(),
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

    pub fn from_filepath(file_path: &str) -> Self {
        let mut history = History::new();
        let header = Regex::new(r"\[.*").unwrap();
        let turn = Regex::new(r"\d+").unwrap();
        let result = Regex::new(r"\[Result").unwrap();
        let game_type = Regex::new(r#"\[GameType "(Base[+MLP]+)"\]"#).unwrap();
        match File::open(file_path) {
            Ok(file) => {
                for line in io::BufReader::new(file).lines().flatten() {
                    let tokens = line.split_whitespace().collect::<Vec<&str>>();
                    if line.len() == 0 {
                        continue;
                    }
                    if game_type.is_match(&line) {
                        let caps = game_type.captures(&line).unwrap();
                        if let Some(mtch) = caps.get(1) {
                            history.game_type = GameType::from_str(mtch.as_str());
                        }
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
            Err(e) => {
                println!("Couldn't open file because: {}", e);
            }
        }
        history
    }

    pub fn write_move(&self, file_name: &str, turn: usize, board_move: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(file_name)
            .expect("game.txt cannot be written to");
        if let Err(e) = write!(
            file,
            "{}. {}\n",
            turn, board_move
        ) {
            panic!("{}", e);
        }
    }

    pub fn write_file(&self, file_name: String) {
        // TODO rewrite this to not open the file for every single line
        for (i, (piece, pos)) in self.moves.iter().enumerate() {
            let mov = format!("{} {}", piece, pos);
            self.write_move(&file_name, i, mov);
        }
    }
}
