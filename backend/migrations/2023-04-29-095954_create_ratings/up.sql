CREATE TABLE ratings (
  id int generated always as identity primary key,
  user_uid text references users(uid) on delete cascade not null,
  game_type text not null,
  games_played int8,
  turn_based float8 not null,
  puzzle float8 not null,
)
