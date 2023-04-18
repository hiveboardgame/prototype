CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  game_type TEXT NOT NULL,
  game_status TEXT NOT NULL,
  turn INT4 NOT NULL DEFAULT 0,
  history TEXT[],
  white TEXT NOT NULL,
  black TEXT NOT NULL,
  tournament_rules BOOLEAN NOT NULL DEFAULT TRUE
);
