use hive_lib::bug::Bug;
use hive_lib::color::Color;
use hive_lib::history::History;
use hive_lib::state::State;

fn main() {
    let history = History::from_filepath("game.txt");
    let state = State::new_from_history(&history);
    println!("{}", state.board);
    println!("{} won!", state.board.winner().unwrap());
}
