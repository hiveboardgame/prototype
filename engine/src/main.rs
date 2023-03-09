use hive_lib::game_error::GameError;
use hive_lib::game_result::GameResult;
use hive_lib::history::History;
use hive_lib::state::State;
use std::env;

fn play_game_from_file(file_path: &str) -> Result<(), GameError> {
    let history = History::from_filepath(file_path)?;
    let state = State::new_from_history(&history)?;
    if let GameResult::Winner(winner) = state.game_result {
        println!("State says {winner} won!");
    }
    if GameResult::Draw == state.game_result {
        println!("State says it's a draw");
    }
    if let GameResult::Winner(winner) = history.result {
        println!("History says {winner} won!");
    }
    if let GameResult::Winner(hw) = history.result {
        if let GameResult::Winner(sw) = state.game_result {
            if sw != hw {
                return Err(GameError::ResultMismatch {
                    reported_result: history.result,
                    actual_result: state.game_result,
                });
            }
        }
        if let GameResult::Draw = state.game_result {
            return Err(GameError::ResultMismatch {
                reported_result: history.result,
                actual_result: state.game_result,
            });
        }
    }
    if let GameResult::Draw = history.result {
        println!("History says game ended in a draw");
        if let GameResult::Winner(_) = state.game_result {
            return Err(GameError::ResultMismatch {
                reported_result: history.result,
                actual_result: state.game_result,
            });
        }
    }
    Ok(())
}

fn main() {
    let game: Vec<String> = env::args().collect();
    if let Some(game) = game.get(1) {
        println!("{game}");
        match play_game_from_file(game) {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        }
    } else {
        eprint!("{}", GameError::NoPgnFile);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_play_games_from_valid_files() {
        for entry in fs::read_dir("./test_pgns/valid/").expect("Should be valid directory") {
            let entry = entry.expect("PGN").path().display().to_string();
            println!("{}", entry);
            assert!(play_game_from_file(&entry).is_ok());
        }
    }
    #[test]
    fn test_play_games_from_invalid_files() {
        for entry in fs::read_dir("./test_pgns/invalid/").expect("Should be valid directory") {
            let entry = entry.expect("PGN").path().display().to_string();
            println!("{}", entry);
            assert!(play_game_from_file(&entry).is_err());
        }
    }
}
