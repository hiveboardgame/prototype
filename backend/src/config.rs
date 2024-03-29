use dotenvy::dotenv;
use std::path::PathBuf;
use std::{env, env::VarError};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_url: String,
    pub firebase_jwt_issuer: String,
    pub static_files_path: PathBuf,
}

impl ServerConfig {
    pub fn from_env() -> Result<ServerConfig, VarError> {
        dotenv().ok();
        Ok(ServerConfig {
            database_url: env::var("DATABASE_URL")?,
            firebase_jwt_issuer: env::var("FIREBASE_JWT_ISSUER")?,
            static_files_path: env::var("STATIC_FILES_PATH")?.into(),
        })
    }

    pub fn from_test_env() -> Result<ServerConfig, VarError> {
        dotenv().ok();
        if cfg!(test) {
            return Ok(ServerConfig {
                database_url: env::var("TEST_DATABASE_URL")?,
                firebase_jwt_issuer: env::var("FIREBASE_JWT_ISSUER")?,
                static_files_path: env::var("STATIC_FILES_PATH")?.into(),
            });
        }
        unreachable!("You called a test function in a non test binary!");
    }
}
