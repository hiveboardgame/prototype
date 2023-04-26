mod api;
mod config;
mod db;
mod extractors;
mod model;
mod server_error;
mod static_files;
mod websockets;

use crate::api::game;
use crate::api::user;
use crate::config::ServerConfig;
use crate::db::util::{get_pool, DbPool};

use actix_web::dev::Server;
use actix_web::web;
use actix_web::{middleware, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use api::game::challenge;
use api::health;
use websockets::echo::Echo;

/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

pub async fn run(address: &str) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = ServerConfig::from_env().expect("Not all env vars are set");

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    let pool: DbPool = get_pool(&config.database_url)
        .await
        .expect("failed to open connection to database");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(echo_ws)))
            .service(
                web::scope("/api")
                    .service(health::health_check)
                    .service(challenge::get_lobby_challenges)
                    .service(challenge::create_game_challenge)
                    .service(challenge::get_game_challenge)
                    .service(challenge::accept_game_challenge)
                    .service(challenge::delete_game_challenge)
                    .service(game::play::game_play)
                    .service(game::show::get_game)
                    .service(user::get_user)
                    .service(user::get_user_challenges)
                    .service(user::get_user_games)
                    .service(user::create_user)
                    .service(user::create_guest_user),
            )
            .service(static_files::static_file_service(
                config.static_files_path.clone(),
            ))
    })
    .workers(4)
    .bind(address)?
    .run();
    Ok(server)
}
