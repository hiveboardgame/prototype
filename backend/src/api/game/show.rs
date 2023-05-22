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

#[cfg(test)]
mod tests {
    use crate::challenge::game_challenge_response::GameChallengeResponse;
    use crate::{accept_challenge, get_game, make_challenge, make_user};
    use crate::{api::game::game_state_response::GameStateResponse, test::DBTest};
    use actix_web::test::{self, TestRequest};
    use serde_json::json;
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(DBTest)]
    #[actix_rt::test]
    #[serial]
    async fn get_game(_ctx: &mut DBTest) {
        let app = test::init_service(crate::new_test_app().await).await;
        let black = make_user!("black", &app);
        let white = make_user!("white", &app);
        let challenge_response = make_challenge!(white.uid.clone(), "White", &app);
        let game = accept_challenge!(challenge_response.id, black.uid.clone(), &app);
        let game_from_endpoint = get_game!(game.game_id, &app);
        assert_eq!(game.game_id, game_from_endpoint.game_id);
    }
}
