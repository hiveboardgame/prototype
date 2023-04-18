CREATE TABLE games_users (
  game_id INTEGER REFERENCES games(id),
  user_id TEXT REFERENCES users(uid),
  PRIMARY KEY(game_id, user_id)
);
