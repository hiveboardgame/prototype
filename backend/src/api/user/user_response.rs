use crate::{
    db::util::DbPool,
    model::{ratings::Rating, user::User},
    server_error::ServerError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub username: String,
    pub uid: String,
    pub rating: u64,
    pub is_guest: bool,
    pub played: i64,
    pub win: i64,
    pub loss: i64,
    pub draw: i64,
}

impl UserResponse {
    pub async fn from_uid(uid: &str, pool: &DbPool) -> Result<Self, ServerError> {
        let user = User::find_by_uid(uid, pool).await?;
        let rating = Rating::for_uid(uid, pool).await?;

        Ok(Self {
            username: user.username,
            uid: user.uid,
            is_guest: user.is_guest,
            rating: rating.rating.floor() as u64,
            played: rating.played,
            win: rating.won,
            loss: rating.lost,
            draw: rating.draw,
        })
    }

    pub async fn from_username(uname: &str, pool: &DbPool) -> Result<Self, ServerError> {
        let user = User::find_by_username(uname, pool).await?;
        let rating = Rating::for_uid(&user.uid, pool).await?;

        Ok(Self {
            username: user.username,
            uid: user.uid,
            is_guest: user.is_guest,
            rating: rating.rating.floor() as u64,
            played: rating.played,
            win: rating.won,
            loss: rating.lost,
            draw: rating.draw,
        })
    }
}
