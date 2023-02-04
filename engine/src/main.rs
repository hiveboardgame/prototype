use hive_lib::history::History;
use hive_lib::state::State;
use std::env;

fn main() {
    let game: Vec<String> = env::args().collect();
    if let Some(game) = game.get(1) {
        println!("{}", game);
        let history = History::from_filepath(game);
        let state = State::new_from_history(&history);
        // println!("Board:");
        // println!("{}", state.board);
        if let Some(_winner) = state.winner {
            //println!("State says {} won!", winner);
        }
        if let Some(_winner) = history.winner {
            //println!("History says {} won!", winner);
        }
        if let Some(hw) = history.winner {
            if let Some(sw) = state.winner {
                if sw != hw {
                    println!("winners don't match");
                }
            }
        }
    } else {
        println!("Please supply a .pgn to evaluate.")
    }
}
