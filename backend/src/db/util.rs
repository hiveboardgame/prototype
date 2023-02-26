use std::{env, env::VarError};
use diesel::{
  result::{Error as DieselError, Error::QueryBuilderError},
};
use bb8::PooledConnection;
use diesel_async::{
  pg::AsyncPgConnection,
  pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
};

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn get_conn(
  pool: &DbPool,
) -> Result<PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>, DieselError> {
  pool.get().await.map_err(|e| QueryBuilderError(e.into()))
}

pub fn get_database_url_from_env() -> Result<String, VarError> {
  env::var("DATABASE_URL")
}
