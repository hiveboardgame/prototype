use crate::db::schema::users;
use crate::db::schema::users::dsl::users as users_table;
use crate::db::util::{get_conn, DbPool};
use diesel::{result::Error, Identifiable, Insertable, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize)]
#[primary_key(uid)]
pub struct User {
    pub uid: String,
    pub username: String,
    pub is_guest: bool,
}

impl User {
    pub async fn find_by_uid(pool: &DbPool, uid: &str) -> Result<User, Error> {
        let conn = &mut get_conn(pool).await?;
        users_table.find(uid).first::<Self>(conn).await
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        self.insert_into(users_table).execute(conn).await?;
        Ok(())
    }
}
