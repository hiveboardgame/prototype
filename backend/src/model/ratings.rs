use crate::db::schema::ratings;
use crate::model::user::User;
use diesel::{
    AsExpression, Associations, FromSqlRow, Identifiable, Insertable, Queryable, SqlType,
    AsChangeset,
};
use serde::{Deserialize, Serialize};
use crate::challenge::game_challenge_response::NewGameChallengeRequest;

use crate::db::schema::{game_challenges, users};
use crate::db::util::{get_conn, DbPool};
use crate::extractors::auth::AuthenticatedUser;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

#[derive(Insertable, Debug)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    user_uid: String,
    played: i64,
    won: i64,
    lost: i64,
    draw: i64,
    rating: f64,
    deviation: f64,
    volatility: f64,
}

impl NewRating {
    pub fn for_uid(user_uid: String) -> Self {
        Self {
            user_uid,
            played: 0,
            won: 0,
            lost: 0,
            draw: 0,
            rating: 1500.0,
            deviation: 350.0,
            volatility: 0.06,
        }
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(User, foreign_key = user_uid))]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: i32,
    pub user_uid: String,
    pub played: i64,
    pub won: i64,
    pub lost: i64,
    pub draw: i64,
    pub rating: f64,
    pub deviation: f64,
    pub volatility: f64,
}
