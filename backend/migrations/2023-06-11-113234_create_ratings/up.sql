create type GameStatistics as (
  played int8,
  won int8,
  lost int8,
  draw int8,
  rating float8,
  deviation float8,
  volatility float8
);

CREATE TABLE ratings (
  id int generated always as identity primary key,
  user_uid text references users(uid) on delete cascade not null,
  -- only PLM will be rated (for now?)
  rated_games_played int8,
  puzzle GameStatistics not null,
  correspondence GameStatistics not null,
  classical GameStatistics not null,
  rapid GameStatistics not null,
  blitz GameStatistics not null,
  bullet GameStatistics not null
);
