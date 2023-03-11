use crate::api::game::challenge::NewChallengeRequest;
use crate::db::schema::{game_challenges, users};
use crate::db::util::{get_conn, DbPool};
use crate::model::user::User;
use chrono::prelude::*;
use chrono::Days;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

static CHALLENGE_EXPIRATION_LENGTH_IN_DAYS: u64 = 1;

#[derive(Insertable, Debug)]
#[diesel(table_name = game_challenges)]
struct NewGameChallenge {
    challenger_uid: String,
    game_type: String,
    ranked: bool,
    public: bool,
    tournament_queen_rule: bool,
    expiration_time: DateTime<Utc>,
}

#[derive(Identifiable, Queryable, Debug)]
#[diesel(table_name = game_challenges)]
pub struct GameChallenge {
    pub id: Uuid,
    pub challenger_uid: String,
    pub game_type: String,
    pub ranked: bool,
    pub public: bool,
    pub tournament_queen_rule: bool,
    pub expiration_time: DateTime<Utc>, // TODO: periodically cleanup expired challanges
}

fn get_expiration_time() -> DateTime<Utc> {
    let duration_until_expired = Days::new(CHALLENGE_EXPIRATION_LENGTH_IN_DAYS);
    Utc::now() + duration_until_expired
}

impl GameChallenge {
    pub async fn create(
        challenger_uid: &str,
        game: &NewChallengeRequest,
        pool: &DbPool,
    ) -> Result<GameChallenge, Error> {
        let conn = &mut get_conn(pool).await?;
        let new_challenge = NewGameChallenge {
            challenger_uid: challenger_uid.to_string(),
            game_type: game.game_type.to_string(),
            ranked: game.ranked,
            public: game.public,
            tournament_queen_rule: true, // This is always true for now
            expiration_time: get_expiration_time(),
        };
        new_challenge
            .insert_into(game_challenges::table)
            .get_result(conn)
            .await
    }

    pub async fn get(id: &Uuid, pool: &DbPool) -> Result<GameChallenge, Error> {
        let conn = &mut get_conn(pool).await?;
        game_challenges::table.find(id).first(conn).await
    }

    pub async fn get_challenger(&self, pool: &DbPool) -> Result<User, Error> {
        let conn = &mut get_conn(pool).await?;
        users::table.find(&self.challenger_uid).first(conn).await
    }

    pub async fn delete(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        diesel::delete(game_challenges::table.find(self.id))
            .execute(conn)
            .await?;
        Ok(())
    }
}
