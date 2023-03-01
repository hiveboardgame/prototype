-- Your SQL goes here
CREATE TABLE users (
    uid text primary key not null,
    username text not null,
    is_guest boolean not null
)
