use crate::db::schema::users;
use crate::db::schema::users::dsl::users as users_table;
use crate::db::util::{get_conn, DbPool};
use diesel::{result::Error, Identifiable, Insertable, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

const MAX_USERNAME_LENGTH: usize = 20;
const VALID_USERNAME_CHARS: &str = "-_";

#[derive(Debug)]
pub enum UserCreationError {
    InvalidUid(String),
    InvalidUsername(String),
}

fn validate_uid(uid: &str) -> Result<(), UserCreationError> {
    if !uid.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(UserCreationError::InvalidUid(format!("invalid uid: {:?}", uid)));
    }
    Ok(())
}

fn validate_username(username: &str) -> Result<(), UserCreationError> {
    if !username.chars().all(|c| c.is_ascii_alphanumeric() || VALID_USERNAME_CHARS.contains(c)) {
        return Err(UserCreationError::InvalidUsername(format!("invalid username characters: {:?}", username)));
    } else if username.len() > MAX_USERNAME_LENGTH {
        return Err(UserCreationError::InvalidUsername(format!("username must be <= {} chars", MAX_USERNAME_LENGTH)));
    }
    Ok(())
}

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(uid))]
pub struct User {
    uid: String,
    username: String,
    pub is_guest: bool,
}

impl User {
    pub fn new(uid: &str, username: &str, is_guest: bool) -> Result<User, UserCreationError> {
        validate_uid(uid)?;
        validate_username(username)?;
        Ok(User {
            uid: uid.into(),
            username: username.into(),
            is_guest,
        })
    }

    pub async fn find_by_uid(pool: &DbPool, uid: &str) -> Result<User, Error> {
        let conn = &mut get_conn(pool).await?;
        users_table.find(uid).first(conn).await
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        self.insert_into(users_table).execute(conn).await?;
        Ok(())
    }
}
