-- Add migration script here

CREATE TABLE IF NOT EXISTS tb_device_data
(
    timestamp INTEGER NOT NULL,
    value     text    NOT NULL,
    device_id INTEGER NOT NULL,
    unit      TEXT    NOT NULL,
    unit_name TEXT    NOT NULL,
    PRIMARY KEY (timestamp, device_id)
);