CREATE TABLE games (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  black_uid TEXT NOT NULL,
  game_status TEXT NOT NULL,
  game_type TEXT NOT NULL,
  history TEXT NOT NULL,
  game_control_history TEXT NOT NULL,
  rated BOOLEAN NOT NULL DEFAULT TRUE,
  tournament_queen_rule BOOLEAN NOT NULL DEFAULT TRUE,
  turn INTEGER NOT NULL DEFAULT 0,
  white_uid TEXT NOT NULL
);
