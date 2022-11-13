use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq)]
pub struct History {
    pub moves: Vec<(String, String)>,
}

impl History {
    pub fn new() -> Self {
        History { moves: Vec::new() }
    }

    pub fn to_string(&self) -> String {
        let mut his = String::new();
        for (i, (piece, pos)) in self.moves.iter().enumerate() {
            his += &format!("{}. {} {}", i+1, piece, pos);
        }
        his
    }

    pub fn record_move(&mut self, piece: String, pos: String) {
        self.moves.push((piece, pos));
    }

    pub fn from_filepath(str: &str) -> Self {
        let mut history = History::new();
        if let Ok(file) = File::open(str) {
            for line in io::BufReader::new(file).lines().flatten() {
                let mov = line.split_whitespace().collect::<Vec<&str>>();
                history.moves.push((
                    mov.first().unwrap().to_string(),
                    mov.last().unwrap().to_string(),
                ));
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
