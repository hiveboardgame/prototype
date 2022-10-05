use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct History {
    pub moves: Vec<(String, String)>,
}

impl History {
    pub fn new() -> Self {
        let mut moves = Vec::new();
        if let Ok(file) = File::open("./game.txt") {
            for line in io::BufReader::new(file).lines().flatten() {
                let mov = line.split_whitespace().collect::<Vec<&str>>();
                moves.push((
                    mov.first().unwrap().to_string(),
                    mov.last().unwrap().to_string(),
                ));
            }
        }
        History { moves }
    }
}
