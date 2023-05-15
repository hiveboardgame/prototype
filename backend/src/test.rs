use crate::config::ServerConfig;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use test_context::AsyncTestContext;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct DBTest {
    pub conn: PgConnection,
}

#[async_trait::async_trait]
impl AsyncTestContext for DBTest {
    async fn setup() -> DBTest {
        // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        let server_config = ServerConfig::from_test_env().expect("Not all env vars are set");
        let database_url = &server_config.database_url;
        let mut conn = PgConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        conn.revert_all_migrations(MIGRATIONS).unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        DBTest { conn }
    }

    async fn teardown(mut self) {
        //self.conn.revert_all_migrations(migrations).unwrap();
    }
}

#[macro_export]
macro_rules! make_user {
    ( $username:expr, $app:expr ) => {
        {
            let request_body = json!({
                "username": $username,
            });
            let req = TestRequest::post()
                .uri("/api/user")
                .set_json(&request_body)
                .insert_header(("x-authentication", $username))
                .to_request();
            let user: $crate::model::user::User = test::call_and_read_body_json($app, req).await;
            assert_eq!(user.username, $username);
            assert_eq!(user.uid, $username);
            user
        }
    };
}

#[macro_export]
macro_rules! make_guest_user {
    ( $uid:expr, $app:expr ) => {{
        let req = TestRequest::post()
            .uri("/api/guest-user")
            .insert_header(("x-authentication", $uid))
            .to_request();
        let user: $crate::model::user::User = test::call_and_read_body_json($app, req).await;
        assert_eq!(user.uid, $uid);
        assert_ne!(user.username, $uid);
        user
    }};
}

#[macro_export]
macro_rules! make_challenge {
    ( $uid:expr, $color_choice:expr, $app:expr ) => {{
        let request_body = json!({
            "public": true,
            "ranked": false,
            "tournamentQueenRule": true,
            "gameType": "MLP",
            "colorChoice": $color_choice,
        });
        let req = TestRequest::post()
            .uri("/api/game/challenge")
            .set_json(&request_body)
            .insert_header(("x-authentication", $uid))
            .to_request();
        let game_challenge_response: GameChallengeResponse =
            test::call_and_read_body_json($app, req).await;
        game_challenge_response
    }};
}

#[macro_export]
macro_rules! accept_challenge {
    ( $challenge_id:expr, $uid:expr, $app:expr ) => {{
        // white user accepts challenge
        let req = TestRequest::post()
            .uri(&format!("/api/game/challenge/{}/accept", $challenge_id))
            .insert_header(("x-authentication", $uid))
            .to_request();
        let game: GameStateResponse = test::call_and_read_body_json($app, req).await;
        game
    }};
}

#[macro_export]
macro_rules! play_turn {
    ( $game_id:expr, $uid:expr, $move:expr, $app:expr ) => {{
        let request_body = json!({ "Turn": $move });
        let req = TestRequest::post()
            .uri(&format!("/api/game/{}/play", $game_id))
            .set_json(&request_body)
            .insert_header(("x-authentication", $uid))
            .to_request();
        let game: GameStateResponse = test::call_and_read_body_json($app, req).await;
        game
    }};
}

#[macro_export]
macro_rules! game_control {
    ( $game_id:expr, $uid:expr, $game_control_action:expr, $game_control_color:expr, $app:expr ) => {{
        let request_body = json!({
            "GameControl": {$game_control_action: $game_control_color},
        });
        let req = TestRequest::post()
            .uri(&format!("/api/game/{}/play", $game_id))
            .set_json(&request_body)
            .insert_header(("x-authentication", $uid))
            .to_request();
        let game: GameStateResponse =
            test::call_and_read_body_json($app, req).await;
        game
    }};
}

#[macro_export]
macro_rules! get_game {
    ( $game_id:expr, $app:expr ) => {{
        // white user accepts challenge
        let req = TestRequest::get()
            .uri(&format!("/api/game/{}", $game_id))
            .to_request();
        let game: GameStateResponse = test::call_and_read_body_json($app, req).await;
        game
    }};
}
