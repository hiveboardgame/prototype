use crate::{
    api::game::game_state_response::GameStateResponse, db::util::DbPool, model::game::Game,
    server_error::ServerError,
};
use actix_web::{
    get,
    web::{self, Path},
    HttpResponse,
};
use hive_lib::{game_type::GameType, history::History, state::State};
use std::str::FromStr;

#[get("/game/{id}")]
pub async fn get_game(
    path: Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let game_id = path.into_inner();
    let game = Game::get(game_id, &pool).await?;
    let mut history = History::new_from_str(game.history.clone())?;
    history.game_type = GameType::from_str(&game.game_type)?;
    let state = State::new_from_history(&history)?;
    let game_state_response = GameStateResponse::new_from(&game, &state, &pool).await?;
    Ok(HttpResponse::Ok().json(game_state_response))
}
