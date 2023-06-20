use crate::db::schema::games;
use crate::db::schema::ratings;
use crate::db::schema::users;
use crate::db::schema::users::dsl::*;
use crate::db::util::{get_conn, DbPool};
use crate::model::challenge::GameChallenge;
use crate::model::game::Game;
use crate::model::games_users::GameUser;
use crate::model::ratings::NewRating;
use crate::server_error::ServerError;
use diesel::{ExpressionMethods,
    query_dsl::BelongingToDsl, result::Error, Identifiable, Insertable, QueryDsl, Queryable,
    SelectableHelper, AsChangeset,
};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

const MAX_USERNAME_LENGTH: usize = 40;
const VALID_USERNAME_CHARS: &str = "-_";

fn valid_uid_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn validate_uid(id: &str) -> Result<(), ServerError> {
    if !id.chars().all(valid_uid_char) {
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

fn validate_username(candidate_username: &str) -> Result<(), ServerError> {
    if !candidate_username.chars().all(valid_username_char) {
        let reason = format!("invalid username characters: {:?}", username);
        return Err(ServerError::UserInputError {
            field: "username".into(),
            reason,
        });
    } else if candidate_username.len() > MAX_USERNAME_LENGTH {
        let reason = format!("username must be <= {} chars", MAX_USERNAME_LENGTH);
        return Err(ServerError::UserInputError {
            field: "username".into(),
            reason,
        });
    }
    Ok(())
}

#[derive(AsChangeset, Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(primary_key(uid))]
pub struct User {
    pub uid: String,
    pub username: String,
    pub is_guest: bool,
}

impl User {
    pub fn new(new_uid: &str, new_username: &str, guest: bool) -> Result<User, ServerError> {
        validate_uid(new_uid)?;
        validate_username(new_username)?;
        Ok(User {
            uid: new_uid.into(),
            username: new_username.into(),
            is_guest: guest,
        })
    }

    pub async fn find_by_uid(id: &str, pool: &DbPool) -> Result<User, Error> {
        let conn = &mut get_conn(pool).await?;
        users.find(id).first(conn).await
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let connection = &mut get_conn(pool).await?;
        connection
            .transaction::<_, diesel::result::Error, _>(|conn| {
                async move {
                    self.insert_into(users::table).execute(conn).await?;
                    let new_rating = NewRating::for_uid(&self.uid);
                    diesel::insert_into(ratings::table)
                        .values(&new_rating)
                        .execute(conn)
                        .await?;
                    Ok(())
                }
                .scope_boxed()
            })
            .await?;
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

    pub async fn update_username(
        &self,
        new_username: &str,
        pool: &DbPool,
    ) -> Result<(), ServerError> {
        let conn = &mut get_conn(pool).await?;
        validate_username(new_username)?;
        diesel::update(users.find(&self.uid))
            .set(username.eq(new_username))
            .execute(conn).await?;
        Ok(())
    }
}
