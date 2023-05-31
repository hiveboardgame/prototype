#[derive(Insertable, Debug)]
#[diesel(table_name = ratings)]
struct NewRating {
    user_uid: String,
    game_type: String,
    games_played: u64,
    turn_based: f32,
    puzzle: f32,
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(User, foreign_key = user_uid))]
#[diesel(table_name = ratings)]
pub struct Rating {
    id: i32,
    user_uid: String,
    game_type: String,
    games_played: i32,
    turn_based: f64,
    puzzle: f64,
}

impl Rating {
    pub fn new(user_uid: &str, game_typ: &str) -> Result<Self, ServerError> {
        Ok(User {
            uid: uid.into(),
            username: username.into(),
            is_guest,
        })
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        self.insert_into(users_table).execute(conn).await?;
        Ok(())
    }
}
