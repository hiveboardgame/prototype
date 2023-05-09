use crate::db::schema::games;
use crate::db::schema::users;
use crate::db::schema::users::dsl::users as users_table;
use crate::db::util::{get_conn, DbPool};
use crate::model::game::Game;
use crate::model::games_users::GameUser;
use crate::server_error::ServerError;
use diesel::{
    query_dsl::BelongingToDsl, result::Error, Identifiable, Insertable, QueryDsl, Queryable,
    SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use super::challenge::GameChallenge;

const MAX_USERNAME_LENGTH: usize = 40;
const VALID_USERNAME_CHARS: &str = "-_";

fn valid_uid_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn validate_uid(uid: &str) -> Result<(), ServerError> {
    if !uid.chars().all(valid_uid_char) {
        return Err(ServerError::UserInputError {
            field: "uid".into(),
            reason: "invalid characters".into(),
        });
    }
    Ok(())
}

fn valid_username_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_USERNAME_CHARS.contains(c)
}

fn validate_username(username: &str) -> Result<(), ServerError> {
    if !username.chars().all(valid_username_char) {
        let reason = format!("invalid username characters: {:?}", username);
        return Err(ServerError::UserInputError {
            field: "username".into(),
            reason,
        });
    } else if username.len() > MAX_USERNAME_LENGTH {
        let reason = format!("username must be <= {} chars", MAX_USERNAME_LENGTH);
        return Err(ServerError::UserInputError {
            field: "username".into(),
            reason,
        });
    }
    Ok(())
}

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(primary_key(uid))]
pub struct User {
    uid: String,
    username: String,
    pub is_guest: bool,
}

impl User {
    pub fn new(uid: &str, username: &str, is_guest: bool) -> Result<User, ServerError> {
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
        println!("in insert");
        let conn = &mut get_conn(pool).await?;
        println!("got conn");
        let res = self.insert_into(users_table).execute(conn).await;
        println!("res is: {:?}", res);
        res?;
        Ok(())
    }

    pub async fn get_challenges(&self, pool: &DbPool) -> Result<Vec<GameChallenge>, Error> {
        let conn = &mut get_conn(pool).await?;
        GameChallenge::belonging_to(self).get_results(conn).await
    }

    pub async fn get_games(&self, pool: &DbPool) -> Result<Vec<Game>, Error> {
        let conn = &mut get_conn(pool).await?;
        GameUser::belonging_to(self)
            .inner_join(games::table)
            .select(Game::as_select())
            .get_results(conn)
            .await
    }
}
