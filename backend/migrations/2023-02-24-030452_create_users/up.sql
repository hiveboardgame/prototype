-- Your SQL goes here
CREATE TABLE users (
    uid text primary key not null,
    username text not null unique,
    is_guest boolean not null
)
