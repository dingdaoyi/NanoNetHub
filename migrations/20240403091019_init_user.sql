-- Add migration script here


CREATE TABLE IF NOT EXISTS tb_user
(
    id   integer primary key autoincrement,
    name text NOT NULL,
    username text NOT NULL,
    password text NOT NULL,
    email  text  NULL,
    UNIQUE (username)
);