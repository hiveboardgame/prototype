mod api;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Serialize, Deserialize};
use hive_lib::history::History;
use hive_lib::state::State;

use api::token::token;

#[derive(Serialize, Deserialize)]
struct Greeting {
    en: String,
    de: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    let hello = Greeting{ en: "hello".to_owned(), de: "Hallo".to_owned() };
    web::Json(hello)
}

//#[get("/game")]
//async fn game() -> impl Responder {
//    let history = History::from_filepath("../engine/game.txt");
//    let state = State::new_from_history(&history);
//    web::Json(state)
//}

#[get("/history")]
async fn history() -> impl Responder {
    let history = History::from_filepath("../engine/game.txt");
    web::Json(history)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("Builder creation failed");
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect("key.pem couldn't be read");
    builder
        .set_certificate_chain_file("cert.pem")
        .expect("cert.pem couldn't be read");

    HttpServer::new(|| {
        // TODO: make this cors a bit better
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080");

        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(history)
            .service(token)
    })
    .bind("127.0.0.1:8081")? //bindssl(..., builder)?
    .run()
    .await
}
