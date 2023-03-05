mod api;
mod config;
mod db;
mod extractors;
mod model;
mod static_files;
mod websockets;
mod server_error;

use actix_web::web;
use actix_web::{middleware, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use websockets::echo::Echo;

use crate::api::board;
use crate::api::user;
use crate::config::ServerConfig;
use crate::db::util::{get_pool, DbPool};

/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = ServerConfig::from_env().expect("Not all env vars are set");

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    let pool: DbPool = get_pool(&config.database_url)
        .await
        .expect("failed to open connection to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
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
            .service(static_files::static_file_service(config.static_files_path.clone()))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
