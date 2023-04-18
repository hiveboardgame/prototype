CREATE TABLE users (
    uid text primary key not null,
    username varchar(40) not null unique,
    is_guest boolean not null
)
