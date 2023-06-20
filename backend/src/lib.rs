mod api;
mod config;
mod db;
mod extractors;
mod model;
mod server_error;
mod static_files;
mod websockets;

#[cfg(test)]
mod test;

use crate::api::game;
use crate::api::user;
use crate::config::ServerConfig;
use crate::db::util::{get_pool, DbPool};
use actix_web::body::MessageBody;
use actix_web::dev::Server;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::web;
use actix_web::{middleware, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use api::game::challenge;
use api::health;
use std::net::TcpListener;
use websockets::echo::Echo;

/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(echo_ws)))
        .service(
            web::scope("/api")
                .service(health::health_check)
                .service(game::lobby::get_lobby_challenges)
                .service(challenge::create::create_game_challenge)
                .service(challenge::show::get_game_challenge)
                .service(challenge::accept::accept_game_challenge)
                .service(challenge::delete::delete_game_challenge)
                .service(game::play::game_play)
                .service(game::show::get_game)
                .service(user::edit::edit_user)
                .service(user::show::get_user)
                .service(user::challenges::get_user_challenges)
                .service(user::games::get_user_games)
                .service(user::create::create_user)
                .service(user::create_guest::create_guest_user),
        );
}

pub async fn new_test_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let server_config = ServerConfig::from_test_env().expect("Not all env vars are set");
    let pool: DbPool = get_pool(&server_config.database_url)
        .await
        .expect("failed to open connection to database");
    App::new()
        .configure(config)
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(server_config.clone()))
        .service(static_files::static_file_service(
            server_config.static_files_path.clone(),
        ))
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server_config = ServerConfig::from_env().expect("Not all env vars are set");
    let port = listener.local_addr().unwrap().port();
    log::info!("starting HTTP server at http://127.0.0.1:{port}");
    let pool: DbPool = get_pool(&server_config.database_url)
        .await
        .expect("failed to open connection to database");
    let server = HttpServer::new(move || {
        App::new()
            .configure(config)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(server_config.clone()))
            .service(static_files::static_file_service(
                server_config.static_files_path.clone(),
            ))
    })
    .workers(4)
    .listen(listener)?
    .run();
    Ok(server)
}
