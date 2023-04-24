CREATE TABLE games_users (
  game_id INT REFERENCES games(id),
  user_uid TEXT REFERENCES users(uid),
  PRIMARY KEY(game_id, user_uid)
);
