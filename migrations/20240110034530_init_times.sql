-- Add migration script here

CREATE TABLE IF NOT EXISTS tb_device_data
(
    create_time DATETIME NOT NULL,
    value       text     NOT NULL,
    device_id   INTEGER  NOT NULL,
    identifier  TEXT     NOT NULL,
    unit        TEXT     NOT NULL,
    unit_name   TEXT     NOT NULL,
    PRIMARY KEY (create_time, device_id, identifier)
);

CREATE TABLE IF NOT EXISTS tb_icon
(
    id   integer primary key autoincrement,
    icon text NOT NULL,
    name text NOT NULL,
    UNIQUE (name)
);
