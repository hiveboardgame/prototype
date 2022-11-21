mod api;
mod websockets;

use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use api::token::token;
use hive_lib::history::History;
use hive_lib::state::State;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use websockets::echo::Echo;

#[derive(Serialize, Deserialize)]
struct Greeting {
    en: String,
    de: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    let hello = Greeting {
        en: "hello".to_owned(),
        de: "Hallo".to_owned(),
    };
    web::Json(hello)
}

#[get("/history")]
async fn history() -> impl Responder {
    let history = History::from_filepath("../engine/game.txt");
    web::Json(history)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8081");

    HttpServer::new(|| {
        let cors = Cors::default().allowed_origin("http://127.0.0.1:8080");

        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(echo_ws)))
            .service(
                web::scope("/api")
                    .wrap(cors)
                    .service(history)
                    .service(hello)
                    .service(echo),
            )
        //.service(token)
    })
    .workers(2)
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
