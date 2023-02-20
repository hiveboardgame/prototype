use hive_lib::history::History;
use hive_lib::game_result::GameResult;
use hive_lib::state::State;
use std::env;

fn main() {
    let game: Vec<String> = env::args().collect();
    if let Some(game) = game.get(1) {
        println!("{}", game);
        let history = History::from_filepath(game);
        let state = State::new_from_history(&history);
        if let GameResult::Winner(winner) = state.game_result {
            println!("State says {} won!", winner);
        }
        if GameResult::Draw == state.game_result {
            println!("State says it's a draw");
        }
        if let GameResult::Winner(winner) = history.result {
            println!("History says {} won!", winner);
        }
        if let GameResult::Winner(hw) = history.result {
            if let GameResult::Winner(sw) = state.game_result {
                if sw != hw {
                    println!("winners don't match");
                }
            }
            if let GameResult::Draw = state.game_result {
                println!("winners don't match");
            }
        }
        if let GameResult::Draw = history.result {
            println!("History says game ended in a draw");
            if let GameResult::Winner(_) = state.game_result {
                panic!("State says there is a winner");
            }
        }
    } else {
        println!("Please supply a .pgn to evaluate.")
    }
}
