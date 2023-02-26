use actix_web::{ post, web, HttpResponse, Responder };
use hive_lib::{history::History, state::State};

#[post("/board/{id}/move/{move}")]
async fn record_move(path: web::Path<(u32, String)>) -> impl Responder {
    let (board_id, board_move) = path.into_inner();
    println!("board_id: {}, move: {}", board_id, board_move);
    let game = "game.txt";
    let history = History::from_filepath(game);
    println!("{:?}", history);
    let mut state = State::new_from_history(&history);
    // TODO this is hacky af
    let tokens = board_move.split_whitespace().collect::<Vec<&str>>();
    let piece = *tokens.get(0).unwrap();
    let position = *tokens.get(1).unwrap();
    state.play_turn_from_notation(piece, position);
    state.history.write_move(game, state.turn, board_move);
    println!("{}", state.board);
    HttpResponse::Ok()
}