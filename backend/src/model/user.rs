use crate::db::schema::users;
use crate::db::schema::users::dsl::users as users_table;
use crate::db::util::{get_conn, DbPool};
use diesel::{result::Error, Identifiable, Insertable, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum UserCreationError {
    InvalidUid(String),
    InvalidUsername(String),
}

#[derive(Debug)]
pub struct Uid {
    inner: String,
}

impl From<&Uid> for String {
    fn from(uid: &Uid) -> Self {
        return uid.inner.clone()
    }
}

impl TryFrom<&str> for Uid {
    type Error = UserCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(Uid { inner: value.to_string() })
        } else {
            Err(UserCreationError::InvalidUid(format!("invalid uid: {:?}", value)))
        }
    }
}

#[derive(Debug)]
pub struct Username {
    inner: String,
}

impl TryFrom<&str> for Username {
    type Error = UserCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(Username { inner: value.to_string() })
        } else {
            Err(UserCreationError::InvalidUsername(format!("invalid username: {:?}", value)))
        }
    }
}

impl From<&Username> for String {
    fn from(username: &Username) -> Self {
        return username.inner.clone()
    }
}

#[derive(Debug)]
pub struct User {
    uid: Uid,
    username: Username,
    pub is_guest: bool,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("User", 3)?;
            state.serialize_field("uid", &self.uid.inner)?;
            state.serialize_field("username", &self.username.inner)?;
            state.serialize_field("is_guest", &self.is_guest)?;
            state.end()
    }
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        UserRow {
            uid: (&user.uid).into(),
            username: (&user.username).into(),
            is_guest: user.is_guest,
        }
    }
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        User {
            uid: Uid { inner: value.uid },
            username: Username { inner: value.username },
            is_guest: value.is_guest,
        }
    }
}

impl User {
    pub fn new(maybe_uid: &str, maybe_username: &str, is_guest: bool) -> Result<User, UserCreationError> {
        let uid = Uid::try_from(maybe_uid)?;
        let username = Username::try_from(maybe_username)?;
        Ok(User {
            uid,
            username,
            is_guest,
        })
    }

    pub async fn find_by_uid(pool: &DbPool, uid: &str) -> Result<User, Error> {
        let conn = &mut get_conn(pool).await?;
        let row = users_table.find(uid).first::<UserRow>(conn).await?;
        Ok(row.into())
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        UserRow::from(self).insert_into(users_table).execute(conn).await?;
        Ok(())
    }
}

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(primary_key(uid))]
struct UserRow {
    pub uid: String,
    pub username: String,
    pub is_guest: bool,
}
