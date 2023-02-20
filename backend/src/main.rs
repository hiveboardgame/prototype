mod api;
mod websockets;

use actix_files::NamedFile;
use actix_files::Files;
use actix_web::{
    get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_actors::ws;
use hive_lib::{history::History, state::State};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use websockets::echo::Echo;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/board/{id}/move/{move}")]
async fn board_record_move(path: web::Path<(u32, String)>) -> impl Responder {
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
 
/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(echo_ws)))
            .service(
                web::scope("/api")
                    .service(echo)
                    .service(board_record_move),
            )
            .service(Files::new("/", "dist/").index_file("index.html"))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
