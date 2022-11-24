mod api;
mod websockets;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    get, http::header, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_actors::ws;
use hive_lib::history::History;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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

async fn index(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./dist/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn js(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./dist/frontend.js".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn wasm(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./dist/frontend_bg.wasm".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        //let cors = Cors::default().allowed_origin("http://127.0.0.1:8080");

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/frontend.js", web::get().to(js))
            .route("/frontend_bg.wasm", web::get().to(wasm))
            //.service(Files::new("/dist", ".").index_file("index.html"))
            .service(web::resource("/ws/").route(web::get().to(echo_ws)))
            .service(
                web::scope("/api")
                    //.wrap(cors)
                    .service(history)
                    .service(hello)
                    .service(echo),
            )
        //.service(token)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
