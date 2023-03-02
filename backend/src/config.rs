use std::path::PathBuf;
use std::{env, env::VarError};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_uri: String,
    pub firebase_jwt_issuer: String,
    pub firebase_jwt_authority: String,
    pub static_files_path: PathBuf,
}

impl ServerConfig {
    pub fn from_env() -> Result<ServerConfig, VarError> {
        Ok(ServerConfig {
            database_uri: env::var("DATABASE_URI")?,
            firebase_jwt_issuer: env::var("FIREBASE_JWT_ISSUER")?,
            firebase_jwt_authority: env::var("FIREBASE_JWT_AUTHORITY")?,
            static_files_path: env::var("STATIC_FILES_PATH")?.into(),
        })
    }
}
