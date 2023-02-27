mod api;
mod db;
mod extractors;
mod model;
mod websockets;

use actix_files::Files;
use actix_web::web;
use actix_web::{middleware, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use diesel_async::{
    pg::AsyncPgConnection,
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
};
use websockets::echo::Echo;

use crate::api::board;
use crate::api::user;
use crate::db::util::{get_database_url_from_env, DbPool};

/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    let db_url = get_database_url_from_env().expect("must specify DATABASE_URL env variable");
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(echo_ws)))
            .service(
                web::scope("/api")
                    .service(board::record_move)
                    .service(user::get_user)
                    .service(user::create_user)
                    .service(user::create_guest_user),
            )
            .service(Files::new("/", "dist/").index_file("index.html"))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
