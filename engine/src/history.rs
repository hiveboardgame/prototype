use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct History {
    pub moves: Vec<(String, String)>,
}

impl History {
    pub fn new() -> Self {
        History { moves: Vec::new() }
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
