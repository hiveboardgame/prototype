use crate::db::schema::ratings;
use crate::db::schema::sql_types::Gamestatistics;
use crate::model::user::User;
use diesel::{
    AsExpression, Associations, FromSqlRow, Identifiable, Insertable, Queryable, SqlType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsExpression, Serialize, Deserialize)]
#[diesel(postgres_type(name = "Gamestatistics"))]
#[diesel(sql_type = Gamestatistics)]
pub struct Statistics {
    games_played: i64,
    rating: f64,
    deviation: f64,
    volatility: f64,
}

impl Statistics {
    pub fn default() -> Self {
        Self {
            games_played: 0,
            rating: 1500.0,
            deviation: 350.0,
            volatility: 0.06,
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    user_uid: String,
    rated_games_played: i64,
    puzzle: Statistics,
    correspondence: Statistics,
    classical: Statistics,
    rapid: Statistics,
    blitz: Statistics,
    bullet: Statistics,
}

impl NewRating {
    pub fn for_uid(user_uid: String) -> Self {
        Self {
            user_uid,
            rated_games_played: 0,
            puzzle: Statistics::default(),
            correspondence: Statistics::default(),
            classical: Statistics::default(),
            rapid: Statistics::default(),
            blitz: Statistics::default(),
            bullet: Statistics::default(),
        }
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(User, foreign_key = user_uid))]
#[diesel(table_name = ratings)]
pub struct Rating {
    id: i64,
    user_uid: String,
    rated_games_played: i64,
    puzzle: Statistics,
    correspondence: Statistics,
    classical: Statistics,
    rapid: Statistics,
    blitz: Statistics,
    bullet: Statistics,
}
