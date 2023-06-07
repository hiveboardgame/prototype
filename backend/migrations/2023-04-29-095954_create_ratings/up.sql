CREATE TABLE ratings (
  id int generated always as identity primary key,
  user_uid text references users(uid) on delete cascade not null,
  -- only PLM will be rated
  rated_games_played int8,
  puzzle float8 not null,
  correspondence float8 not null, -- 1 move every 5 days - 1 move per day
  classical float8 not null, -- 25min+
  rapid float8 not null, -- 8-20min
  blitz float8 not null, -- 4-7min
  bullet float8 not null -- 1-3min
)
