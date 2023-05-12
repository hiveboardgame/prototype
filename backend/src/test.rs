use crate::config::ServerConfig;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use test_context::AsyncTestContext;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

pub struct DBTest {
    pub conn: PgConnection,
}

#[async_trait::async_trait]
impl AsyncTestContext for DBTest {
    async fn setup() -> DBTest {
        let server_config = ServerConfig::from_test_env().expect("Not all env vars are set");
        let database_url = &server_config.database_url;
        let mut conn = PgConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        DBTest { conn }
    }

    async fn teardown(mut self) {
        self.conn.revert_all_migrations(MIGRATIONS).unwrap();
    }
}
