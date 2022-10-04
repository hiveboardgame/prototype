use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct History {
    pub moves: Vec<(String, String)>,
}

impl History {
    pub fn new() -> Self {
        let mut moves = Vec::new();
        if let Ok(file) = File::open("./game.txt") {
            for line in io::BufReader::new(file).lines() {
                if let Ok(l) = line {
                    let mov = l.split_whitespace().collect::<Vec<&str>>();
                    moves.push((mov.first().unwrap().to_string(), mov.last().unwrap().to_string()));
                }
            }
        }
        History { moves: moves }
    }
}
