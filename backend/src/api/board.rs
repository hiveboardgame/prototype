use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    post, web, HttpResponse, Responder,
};
use hive_lib::{history::History, state::State};

#[post("/board/{id}/move/{move}")]
async fn record_move(path: web::Path<(u32, String)>) -> impl Responder {
    // TODO we need to implement some propper error handling for the backend now but in the next PR
    //let (board_id, board_move) = path.into_inner();
    //let game = "game.txt";
    //let history = History::from_filepath(game)
    //    .map_err(|err| ErrorInternalServerError(format!("Could not load history: {:?}", err)));
    //let mut state = State::new_from_history(&history)
    //    .map_err(|err| ErrorInternalServerError(format!("Could not load history: {:?}", err)));
    //// TODO this is hacky af and should most probably use one of the history functions
    //let tokens = board_move.split_whitespace().collect::<Vec<&str>>();
    //let piece = *tokens.get(0).unwrap();
    //let position = *tokens.get(1).unwrap();
    //state.play_turn_from_notation(piece, position);
    //state.history.write_move(game, state.turn, board_move);
    HttpResponse::Ok()
}
