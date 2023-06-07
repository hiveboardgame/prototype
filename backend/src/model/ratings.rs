use crate::db::schema::ratings;
use crate::db::util::{get_conn, DbPool};
use crate::model::user::User;
use diesel::{
    result::Error, AsExpression, Associations, BelongingToDsl, Identifiable, Insertable, QueryDsl,
    Queryable, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    user_uid: String,
    rated_games_played: i64,
    puzzle: f64,
    correspondence: f64,
    classical: f64,
    rapid: f64,
    blitz: f64,
    bullet: f64,
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(User, foreign_key = user_uid))]
#[diesel(table_name = ratings)]
pub struct Rating {
    id: i32,
    user_uid: String,
    rated_games_played: i32,
    puzzle: f64,
    correspondence: f64,
    classical: f64,
    rapid: f64,
    blitz: f64,
    bullet: f64,
}

impl NewRating {
    pub fn new(user_uid: &str) -> Self {
        Self {
            user_uid: user_uid.to_string(),
            rated_games_played: 0,
            puzzle: 1500.0,
            correspondence: 1500.0,
            classical: 1500.0,
            rapid: 1500.0,
            blitz: 1500.0,
            bullet: 1500.0,
        }
    }
}

