use crate::config::ServerConfig;
use crate::db::util::get_conn;
use crate::get_pool;
use crate::run;
use crate::DbPool;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use actix_web::{
    test::{self, TestRequest},
    App,
};
use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use serial_test::serial;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use test_context::{test_context, AsyncTestContext, TestContext};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct MyAsyncContext {
    pub conn: PgConnection,
}

#[async_trait::async_trait]
impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> MyAsyncContext {
        let server_config = ServerConfig::from_test_env().expect("Not all env vars are set");
        let database_url = &server_config.database_url;
        let mut conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        MyAsyncContext { conn }
    }

    async fn teardown(mut self) {
        self.conn.revert_all_migrations(MIGRATIONS).unwrap();
    }
}

